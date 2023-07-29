use redis::AsyncCommands;
use tokio::join;
use tracing::info;

use crate::{models::profile::Profile, state::AppState};

use super::ProfileError;

impl Profile {
    pub async fn from_name(name: &str, state: &AppState) -> Result<Self, ProfileError> {
        let cache_key = format!("n:{name}");
        let mut redis = state.redis.clone();
        let provider = state.get_random_rpc_provider().await;

        info!(name = name, cache_key = cache_key, "Looking up profile for {name}...");

        // If the value is in the cache, return it
        if let Ok(value) = redis.get::<_, String>(&cache_key).await {
            if !value.is_empty() {
                let entry: Self = serde_json::from_str(value.as_str()).unwrap();

                return Ok(entry);
            }

            return Err(ProfileError::NotFound);
        }

        // Do it all
        let (address, avatar, records, display) = join!(
            Self::resolve_address(name, provider.clone()),
            Self::resolve_avatar(name, provider.clone()),
            Self::resolve_records(name, state),
            Self::resolve_display(name, provider.clone())
        );

        let Ok(address) = address else {
            return Err(ProfileError::NotFound);
        };

        let value = Self {
            avatar,
            name: name.to_string(),
            display: display.unwrap_or_else(|| name.to_string()),
            address: Some(format!("{address:?}")),
            records,
        };

        let response = serde_json::to_string(&value).unwrap();

        redis
            .set_ex::<_, _, ()>(&cache_key, &response, 3600)
            .await
            .unwrap();

        Ok(value)
    }
}
