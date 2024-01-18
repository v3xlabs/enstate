use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::response::Redirect;
use enstate_shared::models::lookup::image::Image;
use enstate_shared::models::lookup::ENSLookup;
use futures::TryFutureExt;

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
    let avatar = state
        .service
        .name_from_name_or_address(&name_or_address, query.fresh)
        .and_then(|name| {
            state.service.resolve_from_name_single(
                name,
                Image::from("avatar").to_boxed(),
                query.fresh,
            )
        })
        .await
        .map_err(profile_http_error_mapper)?;

    Ok(Redirect::to(avatar.as_str()))
}
