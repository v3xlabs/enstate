#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod abi;
mod database;
mod http;
mod models;
mod routes;
mod state;
mod provider;
mod utils;

use dotenvy::dotenv;
use state::AppState;
use tracing::{Level, info};
use std::env;

use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("📦 enstate.rs v{}", env!("CARGO_PKG_VERSION"));
    // println!("📦 enstate.rs v{}", env!("CARGO_PKG_VERSION"));

    let state = AppState::new().await;

    http::setup(state).listen(3000).await;
}
