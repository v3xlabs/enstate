use ethers::providers::Middleware;
use redis::AsyncCommands;

use crate::{models::profile::Profile, state::AppState};

use super::ProfileError;

impl Profile {
    pub async fn from_name(name: &str, state: &AppState) -> Result<Self, ProfileError> {
        let cache_key = format!("n:{name}");
        let mut redis = state.redis.clone();

        // If the value is in the cache, return it
        if let Ok(value) = redis.get::<_, String>(&cache_key).await {
            if !value.is_empty() {
                let entry: Self = serde_json::from_str(value.as_str()).unwrap();

                return Ok(entry);
            }

            return Err(ProfileError::NotFound);
        }

        // Get the address from the name

        let address = state.provider.resolve_name(name).await.map_err(|e| {
            println!("Error resolving name: {e:?}");

            ProfileError::NotFound
        })?;

        // Get the avatar from the name
        let avatar = state
            .provider
            .resolve_avatar(name)
            .await
            .ok()
            .map(|result| result.to_string());

        let value = Self {
            avatar,
            name: name.to_string(),
            address: Some(format!("{address:?}")),
        };

        let response = serde_json::to_string(&value).unwrap();

        redis
            .set_ex::<_, _, ()>(&cache_key, &response, 300)
            .await
            .unwrap();

        Ok(value)
    }
}
