#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::env;

use dotenvy::dotenv;
use ethers::prelude::namehash;
use futures::FutureExt;
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use state::AppState;

mod abi;
mod cache;
mod database;
mod http;
mod models;
mod provider;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let filter = EnvFilter::new(format!(
        "enstate={},ethers_ccip_read={}",
        Level::DEBUG,
        Level::DEBUG
    ));

    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        .with_env_filter(filter)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("📦 enstate.rs v{}", env!("CARGO_PKG_VERSION"));

    let shutdown_signal = CancellationToken::new();

    let mut sigint_signal =
        signal(SignalKind::interrupt()).expect("SIGINT handler should've registered");

    let state = AppState::new().await;

    let shutdown_clone = shutdown_signal.clone();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT should be a number");

    let server_future = async move {
        let http_result = http::setup(state).listen(port, shutdown_clone).await;

        if let Err(err) = http_result {
            error!("HTTP server error: {}", err);
        }
    };
    let server = server_future.shared();
    let server_thread = tokio::spawn(server.clone());

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Ctrl-C received, shutting down");
        }
        _ = sigint_signal.recv() => {
            info!("SIGINT received, shutting down");
        },
        _ = server => {
            info!("HTTP server exit");
        }
    }

    shutdown_signal.cancel();

    let _ = server_thread.await;

    info!("exited successfully");
}
