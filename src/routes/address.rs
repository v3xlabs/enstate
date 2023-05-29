use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::models::profile::Profile;

#[derive(Deserialize, IntoParams)]
pub struct CacheQuery {
    fresh: bool,
}

#[utoipa::path(
    get,
    path = "/a/{address}",
    responses(
        (status = 200, description = "Successfully found address", body = Profile),
        (status = BAD_REQUEST, description = "Invalid address."),
        (status = NOT_FOUND, description = "No name was associated with this address."),
        (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address."),
    ),
    params(
        ("address" = String, Path, description = "Address to lookup name data for"),
        CacheQuery,
    )
)]
pub async fn get(
    Path(address): Path<String>,
    query: Option<Query<CacheQuery>>,
    State(state): State<crate::AppState>,
) -> Result<Json<Profile>, StatusCode> {
    let address = address.parse().map_err(|_| StatusCode::BAD_REQUEST)?;

    let profile = Profile::from_address(
        address,
        &state,
        query
            .map(|q: Query<CacheQuery>| q.fresh)
            .unwrap_or_default(),
    )
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(profile))
}
