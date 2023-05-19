use axum::{routing::get, Json, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::routes;
use crate::{oapi::ApiDoc, state::AppState};

pub mod address;
pub mod name;
pub mod records;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(version))
        .route("/a/:address", get(routes::address::get))
        .route("/n/:name", get(routes::name::get))
        .route("/r/:name", get(routes::records::get))
}

#[derive(Debug, serde::Serialize)]
pub struct AppVersion {
    name: String,
    semver: String,
    rev: String,
    compile_time: String,
}

#[allow(clippy::unused_async)]
pub async fn version() -> Json<AppVersion> {
    Json(AppVersion {
        name: env!("CARGO_PKG_NAME").to_string(),
        rev: env!("GIT_REV").to_string(),
        semver: env!("CARGO_PKG_VERSION").to_string(),
        compile_time: env!("STATIC_BUILD_DATE").to_string(),
    })
}
