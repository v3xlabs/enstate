use std::{net::SocketAddr, sync::Arc};

use axum::body::HttpBody;
use axum::routing::MethodRouter;
use axum::{routing::get, Router};
use tokio_util::sync::CancellationToken;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::models::bulk::{BulkResponse, ListResponse};
use crate::models::error::ErrorResponse;
use crate::models::profile::ENSProfile;
use crate::routes;
use crate::state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(routes::address::get, routes::name::get, routes::universal::get),
    components(schemas(ENSProfile, ListResponse<BulkResponse<ENSProfile>>, ErrorResponse))
)]
pub struct ApiDoc;

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

        let server = axum::Server::try_bind(&addr)?
            .serve(self.router.into_make_service())
            .with_graceful_shutdown(async {
                shutdown_signal.cancelled().await;
            });

        info!("Listening HTTP on {}", addr);

        server.await?;

        info!("HTTP server shutdown");

        Ok(())
    }
}

pub fn setup(state: AppState) -> App {
    let router = Router::new()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(routes::root::get))
        .directory_route("/a/:address", get(routes::address::get))
        .directory_route("/n/:name", get(routes::name::get))
        .directory_route("/u/:name_or_address", get(routes::universal::get))
        .directory_route("/i/:name_or_address", get(routes::image::get))
        .directory_route("/h/:name_or_address", get(routes::header::get))
        .directory_route("/bulk/a", get(routes::address::get_bulk))
        .directory_route("/bulk/n", get(routes::name::get_bulk))
        .directory_route("/bulk/u", get(routes::universal::get_bulk))
        .fallback(routes::four_oh_four::handler)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(state));

    App { router }
}

trait RouterExt<S, B>
where
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    fn directory_route(self, path: &str, method_router: MethodRouter<S, B>) -> Self;
}

impl<S, B> RouterExt<S, B> for Router<S, B>
where
    B: HttpBody + Send + 'static,
    S: Clone + Send + Sync + 'static,
{
    fn directory_route(self, path: &str, method_router: MethodRouter<S, B>) -> Self {
        self.route(path, method_router.clone())
            .route(&format!("{path}/"), method_router)
    }
}
