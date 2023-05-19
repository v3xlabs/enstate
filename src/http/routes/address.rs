use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use ethers::{
    providers::{Middleware, ProviderError},
    types::H160,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Response {
    pub name: String,
}

#[utoipa::path(
    get,
    path = "/a/{address}",
    responses(
        (status = 200, description = "Successfully found address", body = AddressResponse),
        (status = BAD_REQUEST, description = "Invalid address."),
        (status = NOT_FOUND, description = "No name was associated with this address."),
        (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address."),
    ),
    params(
        ("address" = String, Path, description = "Address to lookup name data for"),
    )
)]
pub async fn get(
    Path(address): Path<String>,
    State(state): State<crate::AppState>,
) -> Result<Json<Response>, StatusCode> {
    let mut redis = state.redis.clone();

    let address = match H160::from_str(address.as_str()) {
        Ok(address) => address,
        Err(e) => {
            println!("Error parsing address: {:?}", e);

            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let cache_key = format!("a:{:?}", address);

    // Get value from the cache otherwise compute
    let name: String = if let Ok(name) = redis.get(&cache_key).await {
        name
    } else {
        let vx = address;
        let v = state.fallback_provider.lookup_address(vx);

        let result = v.await;

        let result = match result {
            Ok(result) => result,
            Err(error) => {
                if let ProviderError::EnsError(error) = error {
                    println!("ENS Error resolving address: {:?}", error);

                    // Cache the value and expire it after 5 minutes
                    redis.set::<_, _, ()>(&cache_key, "").await.unwrap();
                    redis.expire::<_, ()>(&cache_key, 300).await.unwrap();

                    return Err(StatusCode::NOT_FOUND);
                }

                println!("Error resolving address: {:?}", error);
                return Err(StatusCode::NOT_FOUND);
            }
        };

        // Cache the value and expire it after 5 minutes
        redis.set::<_, _, ()>(&cache_key, &result).await.unwrap();
        redis.expire::<_, ()>(&cache_key, 300).await.unwrap();

        result
    };

    if name.is_empty() {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(Json(Response { name }))
    }
}
