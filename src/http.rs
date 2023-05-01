use std::net::SocketAddr;

use axum::{
    routing::{get},
    Router,
};

pub fn setup() -> Router {
    Router::new()
        .route("/", get(crate::routes::root::get))
        .route("/a/:address", get(crate::routes::address::get))
        .route("/n/:name", get(crate::routes::name::get))
        .route("/r/:name", get(crate::routes::records::get))
}

pub async fn start(app: Router) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
