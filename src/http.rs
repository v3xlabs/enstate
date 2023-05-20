use crate::state::AppState;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::routes;

#[derive(OpenApi)]
#[openapi(
    paths(routes::address::get, routes::name::get),
    components(schemas(crate::models::profile::Profile))
)]
pub struct ApiDoc;

pub struct App {
    router: Router,
}

impl App {
    pub async fn listen(self, port: u16) {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        println!("   Listening on http://{addr}\n");
        //       ^^^ Three spaces here to align with enstate.rs header :)

        axum::Server::bind(&addr)
            .serve(self.router.into_make_service())
            .await
            .unwrap();
    }
}

pub fn setup(state: AppState) -> App {
    let router = Router::new()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(routes::root::get))
        .route("/a/:address", get(routes::address::get))
        .route("/n/:name", get(routes::name::get))
        .with_state(state);

    App { router }
}
