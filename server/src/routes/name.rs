use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use enstate_shared::models::profile::Profile;
use serde::Deserialize;

use crate::cache::RedisCache;

#[derive(Deserialize)]
pub struct NameQuery {
    fresh: Option<bool>,
}

#[utoipa::path(
    get,
    path = "/n/{name}",
    responses(
        (status = 200, description = "Successfully found name.", body = SProfile),
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

    let cache = Box::new(RedisCache::new(state.redis.clone()));
    let rpc = state.provider.get_provider();

    let profile = Profile::from_name(
        &name,
        query.fresh.unwrap_or(false),
        cache,
        rpc,
        &state.profile_records,
        &state.profile_chains,
    )
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(profile))
}
