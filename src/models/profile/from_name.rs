use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};

use ethers::providers::namehash;
use redis::AsyncCommands;
use tracing::info;

use crate::{models::profile::Profile, state::AppState};

use super::error::ProfileError;

impl Profile {
    pub async fn from_name(name: &str, state: &AppState) -> Result<Self, ProfileError> {
        let cache_key = format!("n:{name}");
        let mut redis = state.redis.clone();

        info!(
            name = name,
            cache_key = cache_key,
            "Looking up profile for {name}..."
        );

        // If the value is in the cache, return it
        // if let Ok(value) = redis.get::<_, String>(&cache_key).await {
        //     if !value.is_empty() {
        //         let entry: Self = serde_json::from_str(value.as_str()).unwrap();

        //         return Ok(entry);
        //     }

        //     return Err(ProfileError::NotFound);
        // }

        let provider = state.provider.get_provider();

        let namehash = namehash(name);

        let data = vec![
            Self::calldata_address(&namehash),
            Self::calldata_avatar(&namehash),
            Self::calldata_text(&namehash, "location"),
            Self::calldata_text(&namehash, "display"),
        ];

        let (data, resolver) = Self::resolve_universal(name, data, provider).await?;

        info!("Result {:?}", data);

        let address = Self::decode_address(&data[0]).ok();
        let avatar = Self::decode_avatar(&data[1]).ok();
        let location = Self::decode_text(&data[2]).ok();

        let records = BTreeMap::default();

        

        let display = match Self::decode_text(&data[3]).ok() {
            Some(display) if display.to_lowercase() != name.to_lowercase() => display,
            _ => name.to_string(),
        };

        let value = Self {
            name: name.to_string(),
            address: address.map(|address| format!("{:?}", address)),
            avatar,
            display,

            records,
            fresh: chrono::offset::Utc::now().timestamp_millis(),
            resolver: format!("{:?}", resolver),
        };

        let response = serde_json::to_string(&value).unwrap();

        redis
            .set_ex::<_, _, ()>(&cache_key, &response, 3600)
            .await
            .unwrap();

        Ok(value)
    }
}
