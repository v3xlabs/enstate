use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use enstate_shared::models::profile::Profile;
use tokio::sync::Mutex;

use crate::cache::RedisCache;
use crate::routes::{http_simple_status_error, profile_http_error_mapper, FreshQuery, RouteError};

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
    Query(query): Query<FreshQuery>,
    State(state): State<Arc<Mutex<crate::AppState>>>,
) -> Result<Json<Profile>, RouteError> {
    let address = address
        .parse()
        .map_err(|_| http_simple_status_error(StatusCode::BAD_REQUEST))?;

    let state_cloned = state.clone();
    let mut state = state_cloned.lock().await;

    let cache = Box::new(RedisCache::new(state.redis.clone()));
    let rpc = state
        .provider
        .get_provider()
        .ok_or_else(|| http_simple_status_error(StatusCode::INTERNAL_SERVER_ERROR))?
        .clone();

    let opensea_api_key = &state.opensea_api_key;

    let profile = Profile::from_address(
        address,
        query.fresh.unwrap_or(false),
        cache,
        rpc,
        opensea_api_key,
        &state.profile_records,
        &state.profile_chains,
    )
    .await
    .map_err(profile_http_error_mapper)?;

    Ok(Json(profile))
}
