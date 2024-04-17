use std::{net::SocketAddr, sync::Arc};

use aide::axum::routing::get_with;
use aide::openapi::OpenApi;
use aide::scalar::Scalar;
use aide::{axum::routing::get, axum::ApiRouter};
use axum::{Extension, Router};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
// use utoipa::OpenApi;
// use utoipa_swagger_ui::SwaggerUi;

use crate::models::bulk::{BulkResponse, ListResponse};
use crate::models::error::ErrorResponse;
use crate::models::profile::ENSProfile;
use crate::routes;
use crate::state::AppState;

// #[derive(OpenApi)]
// #[openapi(
//     paths(routes::address::get, routes::name::get, routes::universal::get),
//     components(schemas(ENSProfile, ListResponse<BulkResponse<ENSProfile>>, ErrorResponse))
// )]
// pub struct ApiDoc;

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
    aide::gen::on_error(|error| {
        println!("{error}");
    });

    aide::gen::extract_schemas(true);

    let mut api = OpenApi::default();

    let router = ApiRouter::new()
        // .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .api_route_with(
            "/",
            get(Scalar::new("/docs/openapi.json")
                .with_title("Aide Axum")
                .axum_handler()),
            |p| p.security_requirement("ApiKey"),
        )
        .route("/", get(routes::root::get))
        .api_route("/a/:address", get(routes::address::get))
        .api_route("/n/:name", get(routes::name::get))
        .api_route("/u/:name_or_address", get(routes::universal::get))
        .api_route("/i/:name_or_address", get(routes::image::get))
        .api_route("/h/:name_or_address", get(routes::header::get))
        .api_route("/bulk/a", get(routes::address::get_bulk))
        .api_route("/bulk/n", get(routes::name::get_bulk))
        .api_route("/bulk/u", get(routes::universal::get_bulk))
        .api_route("/sse/a", get(routes::address::get_bulk_sse))
        .api_route("/sse/n", get(routes::name::get_bulk_sse))
        .api_route("/sse/u", get(routes::universal::get_bulk_sse))
        .fallback(routes::four_oh_four::handler)
        .finish_api(&mut api)
        .layer(Extension(Arc::new(api)))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(state));

    App { router }
}
