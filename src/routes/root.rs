use axum::Json;
use std::env;

#[derive(Debug, serde::Serialize)]
pub struct AppVersion {
    rev: String,
    name: String,
    semver: String,
    compile_time: String,
}

#[allow(clippy::unused_async)]
pub async fn get() -> Json<AppVersion> {
    Json(AppVersion {
        rev: env!("GIT_REV").to_string(),
        name: env!("CARGO_PKG_NAME").to_string(),
        semver: env!("CARGO_PKG_VERSION").to_string(),
        compile_time: env!("STATIC_BUILD_DATE").to_string(),
    })
}
