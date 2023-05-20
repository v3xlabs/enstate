#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod abi;
mod database;
mod http;
mod models;
mod routes;
mod state;
mod utils;

use dotenvy::dotenv;
use state::AppState;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("📦 enstate.rs v{}", env!("CARGO_PKG_VERSION"));

    let state = AppState::new().await;

    http::setup(state).listen(3000).await;
}
