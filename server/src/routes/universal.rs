use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use enstate_shared::models::profile::Profile;

use crate::routes::{FreshQuery, profile_http_error_mapper, RouteError, universal_profile_resolve};

#[utoipa::path(
    get,
    path = "/u/{name_or_address}",
    responses(
        (status = 200, description = "Successfully found name or address.", body = SProfile),
        (status = NOT_FOUND, description = "No name or address could be found."),
    ),
    params(
        ("name" = String, Path, description = "Name to lookup the address for."),
    )
)]
pub async fn get(
    Path(name_or_address): Path<String>,
    Query(query): Query<FreshQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<Profile>, RouteError> {
    let profile = universal_profile_resolve(&name_or_address, query.fresh.unwrap_or(false), state)
        .await
        .map_err(profile_http_error_mapper)?;

    Ok(Json(profile))
}
