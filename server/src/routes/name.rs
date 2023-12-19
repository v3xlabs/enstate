use std::sync::Arc;

use axum::http::StatusCode;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use enstate_shared::models::profile::Profile;
use futures::future::try_join_all;
use serde::Deserialize;

use crate::cache;
use crate::routes::{
    http_error, http_simple_status_error, profile_http_error_mapper, validate_bulk_input,
    FreshQuery, Qs, RouteError,
};

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
    .map(|res| Json(res.0.get(0).expect("index 0 should exist").clone()))
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
        (status = 200, description = "Successfully found name.", body = ENSProfile),
        (status = NOT_FOUND, description = "No name could be found.", body = ErrorResponse),
    ),
    params(
        ("name" = String, Path, description = "Name to lookup the name data for."),
    )
)]
pub async fn get_bulk(
    Qs(query): Qs<NameGetBulkQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<Vec<Profile>>, RouteError> {
    let names = query
        .names
        .iter()
        .map(|name| name.to_lowercase())
        .collect::<Vec<_>>();

    let names = validate_bulk_input(&names, 10).ok_or_else(|| {
        http_error(
            StatusCode::BAD_REQUEST,
            "input is too long (expected <= 10)",
        )
    })?;

    let cache = cache::Redis::new(state.redis.clone());
    let rpc = state
        .provider
        .get_provider()
        .ok_or_else(|| http_simple_status_error(StatusCode::INTERNAL_SERVER_ERROR))?
        .clone();

    let opensea_api_key = &state.opensea_api_key;

    let profiles = names
        .iter()
        .map(|name| {
            Profile::from_name(
                name,
                query.fresh.fresh,
                Box::new(cache.clone()),
                rpc.clone(),
                opensea_api_key,
                &state.profile_records,
                &state.profile_chains,
            )
        })
        .collect::<Vec<_>>();

    let joined = try_join_all(profiles)
        .await
        .map_err(profile_http_error_mapper)?;

    Ok(Json(joined))
}
