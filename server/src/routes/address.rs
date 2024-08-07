use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use axum::response::sse::Event;
use axum::response::{IntoResponse, Sse};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use enstate_shared::core::lookup_data::LookupInfo;
use enstate_shared::core::Profile;
use ethers_core::types::Address;
use futures::future::join_all;
use serde::Deserialize;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::models::bulk::{BulkResponse, ListResponse};
use crate::models::sse::SSEResponse;
use crate::routes::{
    http_simple_status_error, profile_http_error_mapper, validate_bulk_input, FreshQuery, Qs,
    RouteError,
};

/// /a/{address}
///
/// Here is an example of a valid request that looks up an address:
/// ```url
/// /a/0x225f137127d9067788314bc7fcc1f36746a3c3B5
/// ```
#[utoipa::path(
    get,
    tag = "Single Profile",
    path = "/a/{address}",
    responses(
        (status = 200, description = "Successfully found address.", body = ENSProfile),
        (status = BAD_REQUEST, description = "Invalid address.", body = ErrorResponse),
        (status = NOT_FOUND, description = "No name was associated with this address.", body = ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address.", body = ErrorResponse),
    ),
    params(
        ("address" = String, Path, description = "Address to lookup name data for"),
    )
)]
pub async fn get(
    Path(address): Path<String>,
    Query(query): Query<FreshQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<Profile>, RouteError> {
    get_bulk(
        Qs(AddressGetBulkQuery {
            fresh: query,
            addresses: vec![address],
        }),
        State(state),
    )
    .await
    .map(|mut res| {
        Result::from(res.0.response.remove(0))
            .map(Json)
            .map_err(RouteError::from)
    })?
}

#[derive(Deserialize)]
pub struct AddressGetBulkQuery {
    // TODO (@antony1060): remove when proper serde error handling
    #[serde(default)]
    addresses: Vec<String>,

    #[serde(flatten)]
    fresh: FreshQuery,
}

/// /bulk/a
///
/// Here is an example of a valid request that looks up multiple addresses:
/// ```url
/// /bulk/a?addresses[]=0x225f137127d9067788314bc7fcc1f36746a3c3B5&addresses[]=0xd577D1322cB22eB6EAC1a008F62b18807921EFBc&addresses[]=0x8F8f07b6D61806Ec38febd15B07528dCF2903Ae7&addresses[]=0x8e8Db5CcEF88cca9d624701Db544989C996E3216&addresses[]=0xb8c2C29ee19D8307cb7255e1Cd9CbDE883A267d5&addresses[]=0xF1F78f308F08fDCAC933124ee8B52A376ff542B4
/// ```
#[utoipa::path(
    get,
    tag = "Bulk Profiles",
    path = "/bulk/a",
    responses(
        (status = 200, description = "Successfully found address.", body = BulkResponse<ENSProfile>),
        (status = BAD_REQUEST, description = "Invalid address.", body = ErrorResponse),
        (status = NOT_FOUND, description = "No name was associated with this address.", body = ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address.", body = ErrorResponse),
    ),
    params(
        ("addresses[]" = Vec<String>, Query, description = "Addresses to lookup name data for"),
    )
)]
pub async fn get_bulk(
    Qs(query): Qs<AddressGetBulkQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<ListResponse<BulkResponse<Profile>>>, RouteError> {
    let addresses =
        validate_bulk_input(&query.addresses, state.service.max_bulk_size)?;

    let addresses = addresses
        .iter()
        .map(|address| address.parse::<Address>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| http_simple_status_error(StatusCode::BAD_REQUEST))?;

    let profiles = addresses
        .iter()
        .map(|address| {
            state
                .service
                .resolve_profile(LookupInfo::Address(*address), query.fresh.fresh)
        })
        .collect::<Vec<_>>();

    let joined = join_all(profiles).await.into();

    Ok(Json(joined))
}

/// /sse/a
///
/// Here is an example of a valid request that looks up multiple addresses:
/// ```url
/// /sse/a?addresses[]=0x225f137127d9067788314bc7fcc1f36746a3c3B5&addresses[]=0xd577D1322cB22eB6EAC1a008F62b18807921EFBc&addresses[]=0x8F8f07b6D61806Ec38febd15B07528dCF2903Ae7&addresses[]=0x8e8Db5CcEF88cca9d624701Db544989C996E3216&addresses[]=0xb8c2C29ee19D8307cb7255e1Cd9CbDE883A267d5&addresses[]=0xF1F78f308F08fDCAC933124ee8B52A376ff542B4
/// ```
#[utoipa::path(
    get,
    tag = "Stream Profiles",
    path = "/sse/a",
    responses(
        (status = 200, description = "Successfully found address.", body = BulkResponse<ENSProfile>),
        (status = BAD_REQUEST, description = "Invalid address.", body = ErrorResponse),
        (status = NOT_FOUND, description = "No name was associated with this address.", body = ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address.", body = ErrorResponse),
    ),
    params(
        ("addresses[]" = Vec<String>, Query, description = "Addresses to lookup name data for"),
    )
)]
pub async fn get_bulk_sse(
    Qs(query): Qs<AddressGetBulkQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> impl IntoResponse {
    let addresses =
        validate_bulk_input(&query.addresses, state.service.max_bulk_size).unwrap();

    let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel::<Result<Event, Infallible>>();

    for address_input in addresses {
        let state_clone = state.clone();
        let event_tx_clone = event_tx.clone();
        tokio::spawn(async move {
            let dne_cache_key = format!("dne:{}", address_input);
            let cached_not_exists = state_clone.service.cache.get(&dne_cache_key).await;

            if let Ok(_) = cached_not_exists {
                let sse_response = SSEResponse {
                    query: address_input,
                    response: BulkResponse::Err(
                        http_simple_status_error(StatusCode::NOT_FOUND).into(),
                    ),
                };

                return event_tx_clone.send(Ok(Event::default()
                    .json_data(sse_response)
                    .expect("json_data should've succeeded")));
            }

            let profile = 'a: {
                let address = address_input.parse::<Address>();

                let Ok(address) = address else {
                    break 'a Err(http_simple_status_error(StatusCode::BAD_REQUEST));
                };

                state_clone
                    .service
                    .resolve_profile(LookupInfo::Address(address), query.fresh.fresh)
                    .await
                    .map_err(profile_http_error_mapper)
            };

            if profile.is_err() {
                state_clone.service.cache.set(&dne_cache_key, "404", state_clone.service.cache_ttl.unwrap_or(600)).await;
            }

            let sse_response = SSEResponse {
                query: address_input,
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

/// /sse/a
/// 
/// Same as the GET version, but using POST with a JSON body instead of query parameters allowing for larger requests.
#[utoipa::path(
    post,
    tag = "Stream Profiles",
    path = "/sse/a",
    responses(
        (status = 200, description = "Successfully found address.", body = BulkResponse<ENSProfile>),
        (status = BAD_REQUEST, description = "Invalid address.", body = ErrorResponse),
        (status = NOT_FOUND, description = "No name was associated with this address.", body = ErrorResponse),
        (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address.", body = ErrorResponse),
    ),
    request_body = AddressGetBulkQuery,
)]
pub async fn post_bulk_sse(
    State(state): State<Arc<crate::AppState>>,
    Json(query): Json<AddressGetBulkQuery>,
) -> impl IntoResponse {
    get_bulk_sse(Qs(query), State(state)).await
}
