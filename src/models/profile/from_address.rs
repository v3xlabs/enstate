use ethers::{providers::Middleware, types::H160};
use redis::{aio::ConnectionManager, AsyncCommands};

use crate::state::AppState;

use super::{Profile, ProfileError};

async fn get_from_redis(cache_key: &str, redis: &mut ConnectionManager, fresh: bool) -> Option<String> {
    if fresh {
        return None;
    }

    redis.get::<_, String>(&cache_key).await.ok()
}

impl Profile {
    pub async fn from_address(
        address: H160,
        state: &AppState,
        fresh: bool,
    ) -> Result<Self, ProfileError> {
        let cache_key = format!("a:{address:?}");
        let mut redis = state.redis.clone();

        // Get value from the cache otherwise compute
        let name = if let Some(name) = get_from_redis(&cache_key, &mut redis, fresh).await {
            name
        } else {
            let result = match state.provider.lookup_address(address).await {
                Ok(result) => result,
                Err(error) => {
                    println!("Error resolving address: {error:?}");

                    // Cache the value, and expire it after 5 minutes
                    redis.set_ex::<_, _, ()>(&cache_key, "", 300).await.unwrap();

                    return Err(ProfileError::NotFound);
                }
            };

            // Cache the value, and expire it after 5 minutes
            redis
                .set_ex::<_, _, ()>(&cache_key, &result, 300)
                .await
                .unwrap();

            result
        };

        if name.is_empty() {
            return Err(ProfileError::NotFound);
        }

        Self::from_name(&name, state, fresh).await
    }
}
