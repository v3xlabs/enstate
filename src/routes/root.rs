use std::env;

use axum::Json;

#[derive(Debug, serde::Serialize)]
pub struct AppVersion {
    name: String,
    semver: String,
    rev: String,
    compile_time: String,
}

#[allow(clippy::unused_async)]
pub async fn get() -> Json<AppVersion> {
    Json(AppVersion {
        name: env!("CARGO_PKG_NAME").to_string(),
        rev: env!("GIT_REV").to_string(),
        semver: env!("CARGO_PKG_VERSION").to_string(),
        compile_time: env!("STATIC_BUILD_DATE").to_string(),
    })
}
