use crate::state::AppState;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct App {
    router: Router,
}

impl App {
    pub async fn listen(self, port: u16) {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));

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
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", crate::oapi::ApiDoc::openapi()))
        .route("/", get(crate::routes::root::get))
        .route("/a/:address", get(crate::routes::address::get))
        .route("/n/:name", get(crate::routes::name::get))
        .route("/r/:name", get(crate::routes::records::get))
        .with_state(state);

    App { router }
}
