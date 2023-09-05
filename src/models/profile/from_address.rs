use ethers::{
    providers::{Middleware, ProviderError},
    types::H160,
};
use redis::AsyncCommands;

use crate::state::AppState;

use super::{error::ProfileError, Profile};

impl Profile {
    pub async fn from_address(address: H160, fresh: bool, state: &AppState) -> Result<Self, ProfileError> {
        let cache_key = format!("a:{address:?}");
        let mut redis = state.redis.clone();

        // Get value from the cache otherwise compute
        let name = if let Ok(name) = redis.get(&cache_key).await {
            name
        } else {
            let result = match state.provider.get_provider().lookup_address(address).await {
                Ok(result) => result,
                Err(error) => {
                    println!("Error resolving address: {error:?}");

                    if let ProviderError::EnsError(_) = error {
                        // Cache the value, and expire it after 5 minutes
                        redis
                            .set_ex::<_, _, ()>(&cache_key, "", 3600)
                            .await
                            .unwrap();
                    };

                    return Err(ProfileError::NotFound);
                }
            };

            // Cache the value, and expire it after 5 minutes
            redis
                .set_ex::<_, _, ()>(&cache_key, &result, 3600)
                .await
                .unwrap();

            result
        };

        if name.is_empty() {
            return Err(ProfileError::NotFound);
        }

        Self::from_name(&name, fresh, state).await
    }
}
