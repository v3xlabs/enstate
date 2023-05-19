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

use crate::models::profile_data::ProfileData;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddressResponse {
    pub name: String,
}

#[utoipa::path(
    get,
    path = "/a/{address}",
    responses(
        (status = 200, description = "Successfully found address", body = ProfileData),
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
) -> Result<Json<ProfileData>, StatusCode> {
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
    let value: String = if let Ok(value) = redis.get(&cache_key).await {
        value
    } else {
        let vx = address;
        let v = state.fallback_provider.lookup_address(vx);

        let result = v.await;

        let result = match result {
            Ok(result) => result,
            Err(error) => match error {
                ProviderError::EnsError(_error) => {
                    println!("ENS Error resolving address: {:?}", _error);

                    // Cache the value
                    let _: () = redis.set(&cache_key, "").await.unwrap();

                    // Expire the value after 5 minutes
                    let _: () = redis.expire(&cache_key, 300).await.unwrap();

                    return Err(StatusCode::NOT_FOUND);
                }
                _ => {
                    println!("Error resolving address: {:?}", error);
                    return Err(StatusCode::NOT_FOUND);
                }
            },
        };

        // Cache the value
        let _: () = redis.set(&cache_key, &result).await.unwrap();

        // Expire the value after 5 minutes
        let _: () = redis.expire(&cache_key, 300).await.unwrap();

        result
    };

    if value.is_empty() {
        return Err(StatusCode::NOT_FOUND);
    }

    match ProfileData::new(&value, &state).await {
        Ok(profile_data) => Ok(Json(profile_data)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
