use axum::Json;
use enstate_shared::meta::{AppMeta, APP_META};

#[allow(clippy::unused_async)]
pub async fn get() -> Json<AppMeta> {
    Json(APP_META.clone())
}
