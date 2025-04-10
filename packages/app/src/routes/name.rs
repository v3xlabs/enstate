use std::convert::Infallible;
use std::sync::Arc;
use std::time::{Duration, Instant};

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

/// /n/{name}
///
/// Here is an example of a valid request that looks up a name:
/// ```url
/// /n/luc.eth
/// ```
#[utoipa::path(
    get,
    tag = "Single Profile",
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
    let s = state.clone();
    s.metrics.name_lookup_total.inc();
    let start = Instant::now();

    let result = get_bulk(
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
    })?;

    s.metrics
        .name_lookup_latency
        .observe(start.elapsed().as_secs_f64());

    result
}

#[derive(Deserialize)]
pub struct NameGetBulkQuery {
    // TODO (@antony1060): remove when proper serde error handling
    #[serde(default)]
    names: Vec<String>,

    #[serde(flatten)]
    fresh: FreshQuery,
}

/// /bulk/n
///
/// Here is an example of a valid request that looks up multiple names:
/// ```url
/// /bulk/n?names[]=luc.eth&names[]=nick.eth&names[]=helgesson.eth&names[]=irc.eth&names[]=khori.eth&names[]=v3x.eth
/// ```
#[utoipa::path(
    get,
    tag = "Bulk Profiles",
    path = "/bulk/n",
    responses(
        (status = 200, description = "Successfully found name.", body = ListButWithLength<BulkResponse<Profile>>),
        (status = NOT_FOUND, description = "No name could be found.", body = ErrorResponse),
    ),
    params(
        ("names[]" = Vec<String>, Query, description = "Names to lookup name data for"),
    )
)]
pub async fn get_bulk(
    Qs(query): Qs<NameGetBulkQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<ListResponse<BulkResponse<Profile>>>, RouteError> {
    let names = validate_bulk_input(&query.names, state.service.max_bulk_size)?;

    let profiles = names
        .into_iter()
        .map(|name| {
            state
                .service
                .resolve_profile(LookupInfo::Name(name), query.fresh.fresh)
        })
        .collect::<Vec<_>>();

    let joined: ListResponse<BulkResponse<Profile>> = join_all(profiles).await.into();

    // TODO: +1 on cache hit popularity discover
    for profile in &joined.response {
        if let BulkResponse::Ok(profile) = profile {
            let _ = state.service.cache.cache_hit(&profile.name).await;
            if let Some(discovery) = &state.service.discovery {
                let _ = discovery.discover_name(profile).await;
            }
        }
    }

    Ok(Json(joined))
}

/// /sse/n
///
/// Here is an example of a valid request that looks up multiple names:
/// ```url
/// /sse/n?names[]=luc.eth&names[]=nick.eth&names[]=helgesson.eth&names[]=irc.eth&names[]=khori.eth&names[]=v3x.eth
/// ```
#[utoipa::path(
    get,
    tag = "Stream Profiles",
    path = "/sse/n",
    responses(
        (status = 200, description = "Successfully found name.", body = ListButWithLength<BulkResponse<Profile>>),
        (status = NOT_FOUND, description = "No name could be found.", body = ErrorResponse),
    ),
    params(
        ("names[]" = Vec<String>, Query, description = "Names to lookup name data for"),
    )
)]
pub async fn get_bulk_sse(
    Qs(query): Qs<NameGetBulkQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> impl IntoResponse {
    let names = validate_bulk_input(&query.names, state.service.max_bulk_size).unwrap();

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

/// /sse/n
///
/// Same as the GET version, but using POST with a JSON body instead of query parameters allowing for larger requests.
#[utoipa::path(
    post,
    tag = "Stream Profiles",
    path = "/sse/n",
    responses(
        (status = 200, description = "Successfully found name.", body = ListButWithLength<BulkResponse<Profile>>),
        (status = NOT_FOUND, description = "No name could be found.", body = ErrorResponse),
    ),
    request_body = NameGetBulkQuery,
)]
pub async fn post_bulk_sse(
    State(state): State<Arc<crate::AppState>>,
    Json(query): Json<NameGetBulkQuery>,
) -> impl IntoResponse {
    get_bulk_sse(Qs(query), State(state)).await
}
