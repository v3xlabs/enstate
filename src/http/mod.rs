use crate::state::AppState;
use axum::Router;
use std::net::SocketAddr;
use utoipa::OpenApi;

mod routes;

#[derive(OpenApi)]
#[openapi(
    paths(routes::name::get, routes::address::get, routes::records::get),
    components(schemas(
        routes::name::Response,
        routes::address::Response,
        routes::records::Response
    ))
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
    let router = routes::router().with_state(state);

    App { router }
}
