#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod abi;
mod database;
mod http;
mod oapi;
mod routes;
mod state;

use anyhow::Result;
use dotenvy::dotenv;
use ethers::prelude::*;
use ethers_ccip_read::CCIPReadMiddleware;
use state::AppState;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    println!("📦 enstate.rs v{}", env!("CARGO_PKG_VERSION"));

    let redis = database::setup().await.expect("Failed to connect to Redis");
    let fallback_provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth").unwrap();
    let provider = CCIPReadMiddleware::new(fallback_provider.clone());

    http::setup(AppState {
        redis,
        provider,
        fallback_provider,
    })
    .listen(3000)
    .await;

    Ok(())
}

// let contract = MyThingssssss::new(H160::from_str("0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85").unwrap(), Arc::new(client));
// let v = contract.balance_of(H160::from_str("0x225f137127d9067788314bc7fcc1f36746a3c3B5").unwrap()).await.unwrap();
// println!("balance: {}", v);
