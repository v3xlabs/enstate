use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::Redirect;

use crate::routes::{
    http_simple_status_error, profile_http_error_mapper, universal_profile_resolve, FreshQuery,
    RouteError,
};

#[utoipa::path(
    get,
    path = "/i/{name_or_address}",
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
) -> Result<Redirect, RouteError> {
    let profile = universal_profile_resolve(&name_or_address, query.fresh.unwrap_or(false), state)
        .await
        .map_err(profile_http_error_mapper)?;

    if let Some(avatar) = profile.avatar {
        return Ok(Redirect::to(avatar.as_str()));
    }

    Err(http_simple_status_error(StatusCode::NOT_FOUND))
}
