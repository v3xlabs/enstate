mod http;
mod routes;
mod database;

#[tokio::main]
async fn main() {
    println!("enstate.rs v{}", env!("CARGO_PKG_VERSION"));

    database::setup().await;

    let router = http::setup();

    http::start(router).await;
}
