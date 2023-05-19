use ethers::providers::{Middleware};
use redis::{AsyncCommands};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProfileData {
    pub name: String,
    pub address: Option<String>,
    pub avatar: Option<String>,
}

pub enum ProfileDataError {
    NotFound,
}

impl ProfileData {
    pub async fn new(name: &str, state: &AppState) -> Result<Self, ProfileDataError> {
        let cache_key = format!("n:{}", name);

        let mut redis = state.redis.clone();

        // Get value from the cache otherwise compute
        if let Ok(value) = redis.get(&cache_key).await as Result<String, _> {
            if value.len() > 0 {
                let entry: Self = serde_json::from_str(value.as_str()).unwrap();

                return Ok(entry);
            }

            return Err(ProfileDataError::NotFound);
        }

        // Get the address from the name
        let address_request = state.provider.resolve_name(name);

        let address = match address_request.await {
            Ok(result) => result,
            Err(e) => {
                println!("Error resolving name: {:?}", e);
                return Err(ProfileDataError::NotFound);
            }
        };

        // Get the avatar from the name
        let avatar_request = state.provider.resolve_avatar(name);

        let avatar = match avatar_request.await.ok() {
            Some(result) => Some(result.to_string()),
            None => None,
        };

        // Create the NameResponse
        let value = Self {
            name: name.to_string(),
            address: Some(format!("{:?}", address)),
            avatar,
        };

        let response = serde_json::to_string(&value).unwrap();

        // Cache the value
        let _: () = redis.set(&cache_key, &response).await.unwrap();

        // Expire the value after 5 minutes
        let _: () = redis.expire(&cache_key, 300).await.unwrap();

        // Return `value` as json string
        Ok(value)
    }
}
