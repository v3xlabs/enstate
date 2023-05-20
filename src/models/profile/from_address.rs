use ethers::{types::H160, providers::{Middleware, ProviderError}};
use redis::AsyncCommands;

use crate::state::AppState;

use super::{Profile, ProfileError};

impl Profile {
    pub async fn from_address(address: H160, state: &AppState) -> Result<Self, ProfileError> {
        let cache_key = format!("a:{address:?}");
        let mut redis = state.redis.clone();

        // Get value from the cache otherwise compute
        let name = if let Ok(name) = redis.get(&cache_key).await {
            name
        } else {
            let result = match state.fallback_provider.lookup_address(address).await {
                Ok(result) => result,
                Err(error) => {
                    println!("Error resolving address: {error:?}");

                    if let ProviderError::EnsError(_) = error {
                        // Cache the value, and expire it after 5 minutes
                        redis.set_ex::<_, _, ()>(&cache_key, "", 300).await.unwrap();
                    };

                    return Err(ProfileError::NotFound);
                }
            };

            // Cache the value, and expire it after 5 minutes
            redis.set_ex::<_, _, ()>(&cache_key, &result, 300).await.unwrap();

            result
        };

        if name.is_empty() {
            return Err(ProfileError::NotFound);
        }

        Self::from_name(&name, state).await
    }
}
