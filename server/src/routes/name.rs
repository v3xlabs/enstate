use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use enstate_shared::models::profile::Profile;

use crate::cache::RedisCache;
use crate::routes::{profile_http_error_mapper, FreshQuery, RouteError};

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
    Query(query): Query<FreshQuery>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Json<Profile>, RouteError> {
    let name = name.to_lowercase();

    let cache = Box::new(RedisCache::new(state.redis.clone()));
    let rpc = state.provider.get_provider();

    let opensea_api_key = &state.opensea_api_key;

    let profile = Profile::from_name(
        &name,
        query.fresh.unwrap_or(false),
        cache,
        rpc,
        opensea_api_key,
        &state.profile_records,
        &state.profile_chains,
    )
    .await
    .map_err(profile_http_error_mapper)?;

    println!("{:?}", profile);

    Ok(Json(profile))
}
