use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use enstate_shared::models::profile::Profile;

use crate::cache;
use crate::routes::{http_simple_status_error, profile_http_error_mapper, FreshQuery, RouteError};

#[utoipa::path(
    get,
    path = "/a/{address}",
    responses(
        (status = 200, description = "Successfully found address.", body = ENSProfile),
        (status = BAD_REQUEST, description = "Invalid address.", body = ErrorResponse),
        (status = NOT_FOUND, description = "No name was associated with this address.", body = ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address.", body = ErrorResponse),
    ),
    params(
        ("address" = String, Path, description = "Address to lookup name data for"),
    )
)]
pub async fn get(
    Path(address): Path<String>,
    Query(query): Query<FreshQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<Profile>, RouteError> {
    let address = address
        .parse()
        .map_err(|_| http_simple_status_error(StatusCode::BAD_REQUEST))?;

    let cache = Box::new(cache::Redis::new(state.redis.clone()));
    let rpc = state
        .provider
        .get_provider()
        .ok_or_else(|| http_simple_status_error(StatusCode::INTERNAL_SERVER_ERROR))?
        .clone();

    let opensea_api_key = &state.opensea_api_key;

    let profile = Profile::from_address(
        address,
        query.fresh,
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
