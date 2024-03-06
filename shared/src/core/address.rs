use ethers::middleware::MiddlewareBuilder;
use ethers::providers::ProviderError;
use ethers_ccip_read::CCIPReadMiddleware;
use ethers_core::types::Address;
use thiserror::Error;
use tracing::instrument;

use crate::core::ENSService;
use crate::core::resolvers::reverse::{resolve_reverse, ReverseResolveError};

#[derive(Error, Debug)]
pub enum AddressResolveError {
    #[error("Primary name not found")]
    NotFound,

    #[error("Cache operation failed: {0}")]
    CacheFail(&'static str),

    #[error("RPC error: {0}")]
    RPCError(#[from] ProviderError),

    #[error("Reverse resolution error: {0}")]
    ReverseResolutionError(#[from] ReverseResolveError),
}

impl ENSService {
    #[instrument(skip_all)]
    pub async fn primary_from_address(
        &self,
        address: &Address,
        fresh: bool,
    ) -> Result<String, AddressResolveError> {
        let cache_key = format!("a:{address:?}");

        let rpc = self.rpc.get_instance();
        let rpc = rpc.wrap_into(CCIPReadMiddleware::new);

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
            let result = resolve_reverse(&rpc, address, &self.universal_resolver)
                .await
                .or_else(|error| {
                    match error {
                        // address doesn't resolve, cache ""
                        ReverseResolveError::MissingPrimaryName => Ok("".to_string()),
                        // yield error up, don't cache
                        _ => Err(error),
                    }
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
