use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use serde::Deserialize;

use enstate_shared::models::profile::Profile;

use crate::{cache::RedisCache};
 
#[derive(Deserialize)]
pub struct NameQuery {
    fresh: Option<bool>,
}

#[debug_handler]
#[utoipa::path(
    get,
    path = "/a/{address}",
    responses(
        (status = 200, description = "Successfully found address", body = SProfile),
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
    Query(query): Query<NameQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<Profile>, StatusCode> {
    let address = address.parse().map_err(|_| StatusCode::BAD_REQUEST)?;

    let cache = Box::new(RedisCache::new(state.redis.clone()));
    let rpc = state.provider.get_provider();

    let profile = Profile::from_address(
        address,
        query.fresh.unwrap_or(false),
        cache,
        rpc,
        &state.profile_records,
        &state.profile_chains,
    )
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(profile))
}
