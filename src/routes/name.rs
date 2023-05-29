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
    path = "/n/{name}",
    responses(
        (status = 200, description = "Successfully found name.", body = Profile),
        (status = NOT_FOUND, description = "No name could be found."),
    ),
    params(
        ("name" = String, Path, description = "Name to lookup the address for."),
        ("fresh" = bool, Query, description = "Whether to use the cache or not.")
    )
)]
pub async fn get(
    Path(name): Path<String>,
    query: Option<Query<CacheQuery>>,
    State(state): State<crate::AppState>,
) -> Result<Json<Profile>, StatusCode> {
    let profile = Profile::from_name(
        &name,
        &state,
        query
            .map(|q: Query<CacheQuery>| q.fresh)
            .unwrap_or_default(),
    )
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(profile))
}
