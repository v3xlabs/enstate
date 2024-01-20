use ethers::providers::{Middleware, ProviderError};
use ethers_core::types::Address;
use thiserror::Error;

use crate::core::ENSService;

#[derive(Error, Debug)]
pub enum AddressResolveError {
    #[error("Primary name not found")]
    NotFound,

    #[error("Cache operation failed: {0}")]
    CacheFail(&'static str),

    #[error("RPC error: {0}")]
    RPCError(#[from] ProviderError),
}

impl ENSService {
    pub async fn primary_from_address(
        &self,
        address: &Address,
        fresh: bool,
    ) -> Result<String, AddressResolveError> {
        let cache_key = format!("a:{address:?}");

        let rpc = self.rpc.get_instance();

        // TODO: improve
        let cached_name = if fresh {
            None
        } else {
            self.cache.get(&cache_key).await.ok()
        };

        // Get value from the cache otherwise compute
        let name = if let Some(name) = cached_name {
            name
        } else {
            let result = rpc
                .lookup_address(*address)
                .await
                .or_else(|error| match error {
                    // address doesn't resolve, cache ""
                    ProviderError::EnsError(_) => Ok("".to_string()),
                    // yield error up, don't cache
                    _ => Err(error),
                })?;

            // Cache the value, and expire it after 10 minutes
            self.cache
                .set(&cache_key, &result, 600)
                .await
                .map_err(|_| AddressResolveError::CacheFail("set"))?;

            result
        };

        if name.is_empty() {
            return Err(AddressResolveError::NotFound);
        }

        Ok(name)
    }
}
