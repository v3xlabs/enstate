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
    get_bulk(
        Qs(NameGetBulkQuery {
            fresh: query,
            names: vec![name],
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
pub struct NameGetBulkQuery {
    // TODO (@antony1060): remove when proper serde error handling
    #[serde(default)]
    names: Vec<String>,

    #[serde(flatten)]
    fresh: FreshQuery,
}

#[utoipa::path(
    get,
    path = "/bulk/n/",
    responses(
        (status = 200, description = "Successfully found name.", body = ListButWithLength<BulkResponse<Profile>>),
        (status = NOT_FOUND, description = "No name could be found.", body = ErrorResponse),
    ),
    params(
        ("name" = String, Path, description = "Name to lookup the name data for."),
    )
)]
pub async fn get_bulk(
    Qs(query): Qs<NameGetBulkQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<ListResponse<BulkResponse<Profile>>>, RouteError> {
    let names = validate_bulk_input(&query.names, 10)?;

    let profiles = names
        .iter()
        .map(|name| state.service.resolve_from_name(name, query.fresh.fresh))
        .collect::<Vec<_>>();

    let joined = join_all(profiles).await.into();

    Ok(Json(joined))
}
