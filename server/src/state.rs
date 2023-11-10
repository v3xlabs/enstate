use enstate_shared::models::{
    multicoin::cointype::{coins::CoinType, Coins},
    records::Records,
};
use redis::aio::ConnectionManager;
use std::env;
use tracing::info;

use crate::{database, provider::RoundRobinProvider};

#[allow(clippy::module_name_repetitions)]
pub struct AppState {
    pub redis: ConnectionManager,
    pub profile_records: Vec<String>,
    pub profile_chains: Vec<CoinType>,
    pub rpc_urls: Vec<String>,
    pub opensea_api_key: String,
    pub provider: RoundRobinProvider,
}

impl AppState {
    pub async fn new() -> Self {
        let profile_records = env::var("PROFILE_RECORDS").map_or_else(
            |_| Records::default().records,
            |s| s.split(",").map(std::string::ToString::to_string).collect(),
        );

        let multicoin_chains: Vec<CoinType> = env::var("MULTICOIN_CHAINS").map_or_else(
            |_| Coins::default().coins,
            |_| {
                // TODO: Implement chain parsing
                vec![]
            }, // |s| s.split(",").map(std::string::ToString::to_string).collect(),
        );

        let raw_rpc_urls: String =
            env::var("RPC_URL").expect("RPC_URL environment variable not found.");
        let rpc_urls = raw_rpc_urls
            .split(',')
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>();

        info!("Connecting to Redis...");

        let redis = database::setup().await.expect("Failed to connect to Redis");

        info!("Connected to Redis");

        let provider = RoundRobinProvider::new(rpc_urls.clone());

        let opensea_api_key = env::var("OPENSEA_API_KEY").expect("OPENSEA_API_KEY not found.");

        Self {
            redis,
            profile_records,
            profile_chains: multicoin_chains,
            opensea_api_key,
            rpc_urls,
            provider,
        }
    }
}
