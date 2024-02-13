use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use axum::response::sse::Event;
use axum::response::{IntoResponse, Sse};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use enstate_shared::core::lookup_data::LookupInfo;
use enstate_shared::core::Profile;
use futures::future::join_all;
use serde::Deserialize;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::models::bulk::{BulkResponse, ListResponse};
use crate::models::sse::SSEResponse;
use crate::routes::{profile_http_error_mapper, validate_bulk_input, FreshQuery, Qs, RouteError};

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
        .into_iter()
        .map(|name| {
            state
                .service
                .resolve_profile(LookupInfo::Name(name), query.fresh.fresh)
        })
        .collect::<Vec<_>>();

    let joined = join_all(profiles).await.into();

    Ok(Json(joined))
}

pub async fn get_bulk_sse(
    Qs(query): Qs<NameGetBulkQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> impl IntoResponse {
    let names = validate_bulk_input(&query.names, 10).unwrap();

    let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel::<Result<Event, Infallible>>();

    for name in names {
        let state_clone = state.clone();
        let event_tx_clone = event_tx.clone();
        tokio::spawn(async move {
            let profile = state_clone
                .service
                .resolve_profile(LookupInfo::Name(name.clone()), query.fresh.fresh)
                .await
                .map_err(profile_http_error_mapper);

            let sse_response = SSEResponse {
                query: name,
                response: profile.into(),
            };

            event_tx_clone.send(Ok(Event::default()
                .json_data(sse_response)
                .expect("json_data should've succeeded")))
        });
    }

    Sse::new(UnboundedReceiverStream::new(event_rx))
        .keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(1)))
}
