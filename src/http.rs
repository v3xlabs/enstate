use std::net::SocketAddr;

use axum::{routing::get, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::AppState;

pub fn setup(state: AppState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", crate::oapi::ApiDoc::openapi()))
        .route("/", get(crate::routes::root::get))
        .route("/a/:address", get(crate::routes::address::get))
        .route("/n/:name", get(crate::routes::name::get))
        .route("/r/:name", get(crate::routes::records::get))
        .with_state(state)
}

pub async fn start(app: Router) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
