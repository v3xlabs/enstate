use std::sync::Arc;

use axum::{
    extract::{Path, State, Query},
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
    path = "/n/{name}",
    responses(
        (status = 200, description = "Successfully found name.", body = ProfileData),
        (status = NOT_FOUND, description = "No name could be found."),
    ),
    params(
        ("name" = String, Path, description = "Name to lookup the address for."),
    )
)]
pub async fn get(
    Path(name): Path<String>,
    Query(query): Query<NameQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<Profile>, StatusCode> {
    let name = name.to_lowercase();

    let profile = Profile::from_name(&name, query.fresh.unwrap_or(false), &state)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(profile))
}
