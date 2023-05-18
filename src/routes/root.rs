use std::env;

use axum::Json;

#[derive(Debug, serde::Serialize)]
pub struct AppVersion {
    semver: String,
    rev: Option<String>,
    compile_time: String,
}

#[allow(clippy::unused_async)]
pub async fn get() -> Json<AppVersion> {
    Json(AppVersion {
        rev: env::var("GIT_REV").ok(),
        semver: env!("CARGO_PKG_VERSION").to_string(),
        compile_time: env!("STATIC_BUILD_DATE").to_string(),
    })
}
