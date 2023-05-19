use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use ethers::providers::Middleware;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NameResponse {
    pub address: String,
    pub avatar: String,
}

#[utoipa::path(
    get,
    path = "/n/{name}",
    responses(
        (status = 200, description = "Successfully found name.", body = NameResponse),
        (status = NOT_FOUND, description = "No name could be found."),
    ),
    params(
        ("name" = String, Path, description = "Name to lookup the address for."),
    )
)]
pub async fn get(
    Path(name): Path<String>,
    State(state): State<crate::AppState>,
) -> Result<String, StatusCode> {
    let mut redis = state.redis.clone();

    let cache_key = format!("n:{}", name);

    // Get value from the cache otherwise compute
    if let Ok(value) = redis.get(&cache_key).await {
        return Ok(value);
    }

    // Get the address from the name
    let address_request = state.provider.resolve_name(name.as_str());

    let address = match address_request.await {
        Ok(result) => result,
        Err(e) => {
            println!("Error resolving name: {:?}", e);
            return Err(StatusCode::NOT_FOUND);
        }
    };

    // Get the avatar from the name
    let avatar_request = state.provider.resolve_avatar(name.as_str());

    let avatar = match avatar_request.await.ok() {
        Some(result) => result.to_string(),
        None => "".to_string(),
    };

    // Create the NameResponse
    let value = NameResponse {
        address: format!("{:?}", address),
        avatar: avatar.to_string(),
    };

    let response = serde_json::to_string(&value).unwrap();

    // Cache the value
    let _: () = redis.set(&cache_key, &response).await.unwrap();

    // Expire the value after 5 minutes
    let _: () = redis.expire(&cache_key, 300).await.unwrap();

    // Return `value` as json string
    Ok(response)
}
