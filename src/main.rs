#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod abi;
mod database;
mod http;
mod routes;
mod state;

use dotenvy::dotenv;
use ethers::prelude::*;
use ethers_ccip_read::CCIPReadMiddleware;
use state::AppState;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("📦 enstate.rs v{}", env!("CARGO_PKG_VERSION"));

    let redis = database::setup().await.expect("Failed to connect to Redis");
    let fallback_provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth").unwrap();
    let provider = CCIPReadMiddleware::new(fallback_provider.clone());

    let state = AppState {
        redis,
        provider,
        fallback_provider,
    };

    http::setup(state).listen(3000).await;
}
