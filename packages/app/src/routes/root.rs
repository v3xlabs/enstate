use axum::Json;
use enstate_shared::meta::{AppMeta, APP_META};

/// Me Endpoint
/// 
/// This Endpoint returns the build information of the running process.
#[utoipa::path(
    get,
    tag = "Deployment Information",
    path = "/this",
    responses(
        (status = 200, description = "", body = AppMeta),
    )
)]
#[allow(clippy::unused_async)]
pub async fn get() -> Json<AppMeta> {
    Json(APP_META.clone())
}
