use ethers::providers::{Middleware, ProviderError};
use ethers_core::types::Address;

use super::{error::ProfileError, Profile, ProfileService};

impl ProfileService {
    // TODO: probably can be written nicer
    pub async fn primary_from_address(
        &self,
        address: Address,
        fresh: bool,
    ) -> Result<String, ProfileError> {
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
                .lookup_address(address)
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
                .map_err(|_| ProfileError::Other("cache set failed".to_string()))?;

            result
        };

        if name.is_empty() {
            return Err(ProfileError::NotFound);
        }

        Ok(name)
    }

    pub async fn resolve_from_address(
        &self,
        address: Address,
        fresh: bool,
    ) -> Result<Profile, ProfileError> {
        let name = self.primary_from_address(address, fresh).await?;

        self.resolve_from_name(&name, fresh).await
    }
}
