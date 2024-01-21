use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::response::Redirect;
use enstate_shared::core::error::ProfileError;
use enstate_shared::core::lookup_data::LookupInfo;
use enstate_shared::models::lookup::ENSLookup;

use crate::routes::{profile_http_error_mapper, FreshQuery, RouteError};

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
    let info = LookupInfo::guess(name_or_address)
        .map_err(|_| profile_http_error_mapper(ProfileError::NotFound))?;

    let avatar = state
        .service
        .resolve_record_simple(info, ENSLookup::Image("avatar".to_string()), query.fresh)
        .await
        .map_err(profile_http_error_mapper)?;

    Ok(Redirect::to(avatar.as_str()))
}
