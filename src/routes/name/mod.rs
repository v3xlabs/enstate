use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use ethers::providers::Middleware;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NameResponse {
    pub address: String,
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
) -> Result<Json<NameResponse>, StatusCode> {
    let mut redis = state.redis.clone();

    let cache_key = format!("n:{}", name);

    // Get value from the cache otherwise compute
    let value: String = if let Ok(value) = redis.get(&cache_key).await {
        value
    } else {
        let v = state.provider.resolve_name(name.as_str());

        let result = v.await;

        let result = match result {
            Ok(result) => result,
            Err(e) => {
                println!("Error resolving name: {:?}", e);
                return Err(StatusCode::NOT_FOUND);
            }
        };

        let value = format!("{:?}", result);

        // Cache the value
        let _: () = redis.set(&cache_key, &value).await.unwrap();

        // Expire the value after 5 minutes
        let _: () = redis.expire(&cache_key, 300).await.unwrap();

        value
    };

    Ok(Json(NameResponse { address: value }))
}
