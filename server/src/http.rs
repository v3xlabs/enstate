use axum::response::Html;
use std::{net::SocketAddr, sync::Arc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::routes;
use crate::state::AppState;

pub struct App {
    router: Router,
}

impl App {
    pub async fn listen(
        self,
        port: u16,
        shutdown_signal: CancellationToken,
    ) -> Result<(), anyhow::Error> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        let listener = TcpListener::bind(&addr).await?;

        async fn await_shutdown(shutdown_signal: CancellationToken) {
            shutdown_signal.cancelled().await;
        }

        let server = axum::serve(listener, self.router.into_make_service())
            .with_graceful_shutdown(await_shutdown(shutdown_signal));

        info!("Listening HTTP on {}", addr);

        server.await?;

        info!("HTTP server shutdown");

        Ok(())
    }
}

pub fn setup(state: AppState) -> App {
    let router = Router::new()
        .route("/", get(scalar_handler))
        .route("/docs/openapi.json", get(crate::docs::openapi))
        // .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", crate::docs::ApiDoc::openapi()))
        .route("/this", get(routes::root::get))
        .route("/a/:address", get(routes::address::get))
        .route("/n/:name", get(routes::name::get))
        .route("/u/:name_or_address", get(routes::universal::get))
        .route("/i/:name_or_address", get(routes::image::get))
        .route("/h/:name_or_address", get(routes::header::get))
        .route("/bulk/a", get(routes::address::get_bulk))
        .route("/bulk/n", get(routes::name::get_bulk))
        .route("/bulk/u", get(routes::universal::get_bulk))
        .route("/sse/a", get(routes::address::get_bulk_sse))
        .route("/sse/n", get(routes::name::get_bulk_sse))
        .route("/sse/u", get(routes::universal::get_bulk_sse))
        .fallback(routes::four_oh_four::handler)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(state));

    App { router }
}

// Loads from docs/index.html with headers html
async fn scalar_handler() -> Html<&'static str> {
    let contents = include_str!("./docs/index.html");
    axum::response::Html(contents)
}
