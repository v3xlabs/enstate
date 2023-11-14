use ethers::{
    providers::{Http, Middleware, Provider, ProviderError},
    types::H160,
};

use crate::models::multicoin::cointype::coins::CoinType;

use super::{error::ProfileError, Profile};

impl Profile {
    pub async fn from_address(
        address: H160,
        fresh: bool,
        cache: Box<dyn crate::cache::CacheLayer>,
        rpc: Provider<Http>,
        opensea_api_key: &str,
        profile_records: &[String],
        profile_chains: &[CoinType],
    ) -> Result<Self, ProfileError> {
        let cache_key = format!("a:{address:?}");

        // Get value from the cache otherwise compute
        let name = if let Ok(name) = cache.get(&cache_key).await {
            name
        } else {
            let result = match rpc.lookup_address(address).await {
                Ok(result) => result,
                Err(error) => {
                    println!("Error resolving address: {error:?}");

                    if let ProviderError::EnsError(_) = error {
                        // Cache the value, and expire it after 10 minutes
                        cache
                            .set(&cache_key, "", 600)
                            .await
                            .map_err(|_| ProfileError::Other("cache set failed".to_string()))?;
                    };

                    return Err(ProfileError::NotFound);
                }
            };

            // Cache the value, and expire it after 10 minutes
            cache
                .set(&cache_key, &result, 600)
                .await
                .map_err(|_| ProfileError::Other("cache set failed".to_string()))?;

            result
        };

        if name.is_empty() {
            return Err(ProfileError::NotFound);
        }

        Self::from_name(
            &name,
            fresh,
            cache,
            rpc,
            opensea_api_key,
            profile_records,
            profile_chains,
        )
        .await
    }
}
