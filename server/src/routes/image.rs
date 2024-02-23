use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{AppendHeaders, IntoResponse, Redirect};
use enstate_shared::core::error::ProfileError;
use enstate_shared::core::lookup_data::LookupInfo;
use enstate_shared::models::lookup::ENSLookup;

use crate::routes::{FreshQuery, http_simple_status_error, profile_http_error_mapper, RouteError};

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
) -> Result<impl IntoResponse, RouteError> {
    let info = LookupInfo::guess(name_or_address)
        .map_err(|_| profile_http_error_mapper(ProfileError::NotFound))?;

    let avatar = state
        .service
        .resolve_record_simple(info, ENSLookup::StaticImage("avatar"), query.fresh)
        .await
        .map_err(profile_http_error_mapper)?;

    if let Some(processed) = enstate_shared::utils::data_url::process_data_url_image(&avatar) {
        let Ok(processed) = processed else {
            return Err(http_simple_status_error(StatusCode::UNSUPPORTED_MEDIA_TYPE).into());
        };

        return Ok((
            AppendHeaders([(CONTENT_TYPE, processed.mimetype)]),
            processed.data,
        )
            .into_response());
    }

    Ok(Redirect::to(avatar.as_str()).into_response())
}
