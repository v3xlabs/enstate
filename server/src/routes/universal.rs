use std::sync::Arc;

use axum::http::StatusCode;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use enstate_shared::models::profile::Profile;

use crate::routes::{
    http_simple_status_error, profile_http_error_mapper, universal_profile_resolve, FreshQuery,
    RouteError,
};

#[utoipa::path(
    get,
    path = "/u/{name_or_address}",
    responses(
        (status = 200, description = "Successfully found name or address.", body = ENSProfile),
        (status = NOT_FOUND, description = "No name or address could be found.", body = ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address.", body = ErrorResponse),
    ),
    params(
        ("name_or_address" = String, Path, description = "Name or address to lookup the name data for."),
    )
)]
pub async fn get(
    Path(name_or_address): Path<String>,
    Query(query): Query<FreshQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<Profile>, RouteError> {
    let rpc = state
        .provider
        .get_provider()
        .ok_or_else(|| http_simple_status_error(StatusCode::INTERNAL_SERVER_ERROR))?
        .clone();

    let profile = universal_profile_resolve(&name_or_address, query.fresh, rpc, &state)
        .await
        .map_err(profile_http_error_mapper)?;

    Ok(Json(profile))
}
