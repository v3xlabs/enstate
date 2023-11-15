use std::sync::Arc;

use axum::http::StatusCode;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use enstate_shared::models::profile::Profile;

use crate::cache;
use crate::routes::{http_simple_status_error, profile_http_error_mapper, FreshQuery, RouteError};

#[utoipa::path(
    get,
    path = "/n/{name}",
    responses(
        (status = 200, description = "Successfully found name.", body = ENSProfile),
        (status = NOT_FOUND, description = "No name could be found.", body = ErrorResponse),
    ),
    params(
        ("name" = String, Path, description = "Name to lookup the name data for."),
    )
)]
pub async fn get(
    Path(name): Path<String>,
    Query(query): Query<FreshQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<Profile>, RouteError> {
    let name = name.to_lowercase();

    let cache = Box::new(cache::Redis::new(state.redis.clone()));
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
