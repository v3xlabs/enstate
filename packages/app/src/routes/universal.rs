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
use serde::Deserialize;
use tokio_stream::wrappers::UnboundedReceiverStream;
use utoipa::openapi::schema;
use utoipa::{IntoParams, ToSchema};

use crate::models::bulk::{BulkResponse, ListResponse};
use crate::models::sse::SSEResponse;
use crate::routes::{profile_http_error_mapper, validate_bulk_input, FreshQuery, Qs, RouteError};

/// /u/{name_or_address}
///
/// The Universal Endpoint supports looking up both names and addresses.
///
/// Here is an example of a valid request that looks up an address or name:
/// ```url
/// /u/luc.eth
/// /u/0x225f137127d9067788314bc7fcc1f36746a3c3B5
/// ```
///
/// You can also use the [useProfile](https://github.com/v3xlabs/use-enstate/blob/master/src/hooks/useProfile.ts) hook from [use-enstate](https://github.com/v3xlabs/use-enstate).
#[utoipa::path(
    get,
    tag = "Single Profile",
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
    let response = get_bulk(
        Qs(UniversalGetBulkQuery {
            fresh: query,
            queries: vec![name_or_address],
        }),
        State(state.clone()),
    )
    .await;

    match response {
        Ok(mut res) => {
            for profile in &res.response {
                if let BulkResponse::Ok(profile) = profile {
                    let profile = profile.clone();
                    let _ = state.service.cache.cache_hit(&profile.name).await;
                    if let Some(discovery) = &state.service.discovery {
                        let _ = discovery.discover_name(&profile).await;
                    }
                }
            }

            Result::<_, _>::from(res.response.remove(0))
            .map(Json)
            .map_err(RouteError::from)
        }
        Err(e) => Err(e),
    }
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct UniversalGetBulkQuery {
    // TODO (@antony1060): remove when proper serde error handling
    // list of names or addresses to lookup
    #[schema(example = json!(["luc.eth", "nick.eth", "helgesson.eth", "irc.eth", "khori.eth", "v3x.eth"]))]
    #[serde(default)]
    queries: Vec<String>,

    #[schema(example = "false")]
    #[serde(flatten)]
    fresh: FreshQuery,
}

/// /bulk/u
///
/// The Universal Endpoint supports looking up both names and addresses.
///
/// Here is an example of a valid request that looks up multiple names:
/// ```url
/// /bulk/u?queries[]=luc.eth&queries[]=nick.eth&queries[]=helgesson.eth&queries[]=irc.eth&queries[]=khori.eth&queries[]=v3x.eth
/// ```
///
/// You can also use the [useBulkProfile](https://github.com/v3xlabs/use-enstate/blob/master/src/hooks/useBulkProfile.ts) hook from [use-enstate](https://github.com/v3xlabs/use-enstate).
#[utoipa::path(
    get,
    tag = "Bulk Profiles",
    path = "/bulk/u",
    responses(
        (status = 200, description = "Successfully found name or address.", body = BulkResponse<ENSProfile>),
        (status = NOT_FOUND, description = "No name or address could be found.", body = ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address.", body = ErrorResponse),
    ),
    params(
        ("queries[]" = Vec<String>, Query, description = "Names to lookup name data for"),
    )
)]
pub async fn get_bulk(
    Qs(query): Qs<UniversalGetBulkQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<ListResponse<BulkResponse<Profile>>>, RouteError> {
    let queries = validate_bulk_input(&query.queries, state.service.max_bulk_size)?;

    let profiles = queries
        .iter()
        .map(|input| {
            profile_from_lookup_guess(LookupInfo::guess(input), &state.service, query.fresh.fresh)
        })
        .collect::<Vec<_>>();

    let joined = join_all(profiles).await.into();

    Ok(Json(joined))
}

/// /sse/u
///
/// The Universal Endpoint supports looking up both names and addresses.
///
/// Here is an example of a valid request that looks up multiple names:
/// ```url
/// /sse/u?queries[]=luc.eth&queries[]=nick.eth&queries[]=helgesson.eth&queries[]=irc.eth&queries[]=khori.eth&queries[]=v3x.eth
/// ```
///
/// You can also use the [useSSEProfiles](https://github.com/v3xlabs/use-enstate/blob/master/src/hooks/useSSEProfiles.ts) hook from [use-enstate](https://github.com/v3xlabs/use-enstate).
#[utoipa::path(
    get,
    tag = "Stream Profiles",
    path = "/sse/u",
    responses(
        (status = 200, description = "Successfully found name or address.", body = BulkResponse<ENSProfile>),
        (status = NOT_FOUND, description = "No name or address could be found.", body = ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address.", body = ErrorResponse),
    ),
    params(
        ("queries[]" = Vec<String>, Query, description = "Names to lookup name data for"),
    )
)]
pub async fn get_bulk_sse(
    Qs(query): Qs<UniversalGetBulkQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> impl IntoResponse {
    let queries = validate_bulk_input(&query.queries, state.service.max_bulk_size).unwrap();

    let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel::<Result<Event, Infallible>>();

    for input in queries {
        let state_clone = state.clone();
        let event_tx_clone = event_tx.clone();
        tokio::spawn(async move {
            let profile = profile_from_lookup_guess(
                LookupInfo::guess(&input),
                &state_clone.service,
                query.fresh.fresh,
            )
            .await
            .map_err(profile_http_error_mapper);

            let sse_response = SSEResponse {
                query: input,
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

/// /sse/u
///
/// Same as the GET version, but using POST with a JSON body instead of query parameters allowing for larger requests.
#[utoipa::path(
    post,
    tag = "Stream Profiles",
    path = "/sse/u",
    responses(
        (status = 200, description = "Successfully found name or address.", body = BulkResponse<ENSProfile>),
        (status = NOT_FOUND, description = "No name or address could be found.", body = ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address.", body = ErrorResponse),
    ),
    request_body = UniversalGetBulkQuery,
)]
pub async fn post_bulk_sse(
    State(state): State<Arc<crate::AppState>>,
    Json(query): Json<UniversalGetBulkQuery>,
) -> impl IntoResponse {
    get_bulk_sse(Qs(query), State(state)).await
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
