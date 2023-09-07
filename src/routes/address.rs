use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::models::profile::Profile;

#[derive(Deserialize)]
pub struct NameQuery {
    fresh: Option<bool>,
}

#[utoipa::path(
    get,
    path = "/a/{address}",
    responses(
        (status = 200, description = "Successfully found address", body = ProfileData),
        (status = BAD_REQUEST, description = "Invalid address."),
        (status = NOT_FOUND, description = "No name was associated with this address."),
        (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address."),
    ),
    params(
        ("address" = String, Path, description = "Address to lookup name data for"),
    )
)]
pub async fn get(
    Path(address): Path<String>,
    Query(query): Query<NameQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<Profile>, StatusCode> {
    let address = address.parse().map_err(|_| StatusCode::BAD_REQUEST)?;

    let profile = Profile::from_address(address, query.fresh.unwrap_or(false), &state)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(profile))
}
