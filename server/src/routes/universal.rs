use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use axum::response::sse::Event;
use axum::response::{IntoResponse, Sse};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use enstate_shared::core::error::ProfileError;
use enstate_shared::core::lookup_data::{LookupInfo, NameParseError};
use enstate_shared::core::{ENSService, Profile};
use futures::future::join_all;
use futures::TryFutureExt;
use serde::Deserialize;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::models::bulk::{BulkResponse, ListResponse};
use crate::routes::{profile_http_error_mapper, validate_bulk_input, FreshQuery, Qs, RouteError};

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
            profile_from_lookup_guess(LookupInfo::guess(input), &state.service, query.fresh.fresh)
        })
        .collect::<Vec<_>>();

    let joined = join_all(profiles).await.into();

    Ok(Json(joined))
}

struct SSEResponse {}

pub async fn get_bulk_sse(
    Qs(query): Qs<UniversalGetBulkQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> impl IntoResponse {
    // TODO:
    let queries = validate_bulk_input(&query.queries, 10).unwrap();

    let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel::<Result<Event, Infallible>>();

    for input in queries {
        let state_clone = state.clone();
        let event_tx_clone = event_tx.clone();
        tokio::spawn(async move {
            let profile = profile_from_lookup_guess(
                LookupInfo::guess(input),
                &state_clone.service,
                query.fresh.fresh,
            )
            .await;

            let json = match profile {
                Ok(profile) => serde_json::to_string(&profile).expect("a"),
                Err(err) => serde_json::to_string(&profile_http_error_mapper(err)).expect("b"),
            };

            event_tx_clone.send(Ok(Event::default().data(json)))
        });
    }

    Sse::new(UnboundedReceiverStream::new(event_rx))
        .keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(1)))
}

// helper function for above
async fn profile_from_lookup_guess(
    lookup: Result<LookupInfo, NameParseError>,
    service: &ENSService,
    fresh: bool,
) -> Result<Profile, ProfileError> {
    let Ok(lookup) = lookup else {
        return Err(ProfileError::NotFound);
    };

    service.resolve_profile(lookup, fresh).await
}
