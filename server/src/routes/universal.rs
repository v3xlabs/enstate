use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use enstate_shared::models::profile::Profile;
use futures::future::join_all;
use serde::Deserialize;

use crate::models::bulk::{BulkResponse, ListResponse};
use crate::routes::{validate_bulk_input, FreshQuery, Qs, RouteError};

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
    get_bulk(
        Qs(UniversalGetBulkQuery {
            fresh: query,
            queries: vec![name_or_address],
        }),
        State(state),
    )
    .await
    .map(|mut res| {
        Result::<_, _>::from(res.0.response.remove(0))
            .map(Json)
            .map_err(RouteError::from)
    })?
}

#[derive(Deserialize)]
pub struct UniversalGetBulkQuery {
    // TODO (@antony1060): remove when proper serde error handling
    #[serde(default)]
    queries: Vec<String>,

    #[serde(flatten)]
    fresh: FreshQuery,
}

#[utoipa::path(
    get,
    path = "/bulk/u/",
    responses(
        (status = 200, description = "Successfully found name or address.", body = BulkResponse<ENSProfile>),
        (status = NOT_FOUND, description = "No name or address could be found.", body = ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address.", body = ErrorResponse),
    ),
    params(
        ("name_or_address" = String, Path, description = "Name or address to lookup the name data for."),
    )
)]
pub async fn get_bulk(
    Qs(query): Qs<UniversalGetBulkQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<ListResponse<BulkResponse<Profile>>>, RouteError> {
    let queries = validate_bulk_input(&query.queries, 10)?;

    let profiles = queries
        .iter()
        .map(|input| {
            state
                .service
                .resolve_from_name_or_address(input, query.fresh.fresh)
        })
        .collect::<Vec<_>>();

    let joined = join_all(profiles).await.into();

    Ok(Json(joined))
}
