use std::sync::Arc;

use axum::http::StatusCode;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use enstate_shared::models::profile::Profile;
use tokio::sync::Mutex;

use crate::cache::RedisCache;
use crate::routes::{http_simple_status_error, profile_http_error_mapper, FreshQuery, RouteError};

#[utoipa::path(
    get,
    path = "/n/{name}",
    responses(
        (status = 200, description = "Successfully found name.", body = SProfile),
        (status = NOT_FOUND, description = "No name could be found."),
    ),
    params(
        ("name" = String, Path, description = "Name to lookup the address for."),
    )
)]
pub async fn get(
    Path(name): Path<String>,
    Query(query): Query<FreshQuery>,
    State(state): State<Arc<Mutex<crate::AppState>>>,
) -> Result<Json<Profile>, RouteError> {
    let name = name.to_lowercase();

    let state_cloned = state.clone();
    let mut state = state_cloned.lock().await;

    let cache = Box::new(RedisCache::new(state.redis.clone()));
    let rpc = state
        .provider
        .get_provider()
        .ok_or_else(|| http_simple_status_error(StatusCode::INTERNAL_SERVER_ERROR))?
        .clone();

    let opensea_api_key = &state.opensea_api_key;

    let profile = Profile::from_name(
        &name,
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
