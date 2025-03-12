use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use axum::response::sse::Event;
use axum::response::{IntoResponse, Sse};
use axum::{routing::get, Router};
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
use tracing::info;

use crate::models::bulk::{BulkResponse, ListResponse};
use crate::models::sse::SSEResponse;
use crate::routes::{
    http_simple_status_error, profile_http_error_mapper, validate_bulk_input, FreshQuery, Qs,
    RouteError,
};


pub fn setup_v2_router(state: Arc<crate::AppState>) -> Router<Arc<crate::AppState>> {
    Router::new()
        .route("/discover/search", get(discovery_search)).with_state(state)
}

#[derive(Deserialize)]
pub struct SearchQuery {
    s: String,
}

/// /a/{address}
///
/// Here is an example of a valid request that looks up an address:
/// ```url
/// /a/0x225f137127d9067788314bc7fcc1f36746a3c3B5
/// ```
#[utoipa::path(
    get,
    tag = "Single Profile",
    path = "/v2/discover/search",
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
pub async fn discovery_search(
    Query(query): Query<SearchQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<Vec<Profile>>, RouteError> {

    info!("query: {:?}", query.s);
    
    if let Some(discovery) = &state.service.discovery {
        let profiles = discovery.query_search(&state.service, query.s).await.unwrap();
        return Ok(Json(profiles));
    }
    // get_bulk(
    //     Qs(AddressGetBulkQuery {
    //         fresh: query,
    //         addresses: vec![address],
    //     }),
    //     State(state),
    // )
    // .await
    // .map(|mut res| {
    //     Result::from(res.0.response.remove(0))
    //         .map(Json)
    //         .map_err(RouteError::from)
    // })?
    todo!()
}
