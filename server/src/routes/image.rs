use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::Redirect;

use crate::routes::{http_simple_status_error, profile_http_error_mapper, FreshQuery, RouteError};

// #[utoipa::path(
//     get,
//     path = "/i/{name_or_address}",
//     responses(
//         TODO: figure out body
//         (status = 200, description = "Successfully found name or address.", body = ENSProfile),
//         (status = NOT_FOUND, description = "No name or address could be found."),
//         (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address.", body = ErrorResponse),
//     ),
//     params(
//         ("name_or_address" = String, Path, description = "Name or address to lookup the image for."),
//     )
// )]
pub async fn get(
    Path(name_or_address): Path<String>,
    Query(query): Query<FreshQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Redirect, RouteError> {
    let profile = state
        .service
        .resolve_from_name_or_address(&name_or_address, query.fresh)
        .await
        .map_err(profile_http_error_mapper)?;

    let Some(avatar) = profile.avatar else {
        return Err(http_simple_status_error(StatusCode::NOT_FOUND));
    };

    Ok(Redirect::to(avatar.as_str()))
}
