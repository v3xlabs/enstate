use ethers::{
    providers::{Middleware, ProviderError},
    types::H160,
};

use super::{error::ProfileError, Profile, ProfileService};

impl ProfileService {
    pub async fn resolve_from_address(
        &self,
        address: H160,
        fresh: bool,
    ) -> Result<Profile, ProfileError> {
        let cache_key = format!("a:{address:?}");

        let rpc = self.rpc.get_instance();

        // Get value from the cache otherwise compute
        let name = if let Ok(name) = self.cache.get(&cache_key).await {
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

        self.resolve_from_name(&name, fresh).await
    }
}
