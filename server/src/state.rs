use std::env;
use std::sync::Arc;

use enstate_shared::models::profile::ProfileService;
use enstate_shared::models::{
    multicoin::cointype::{coins::CoinType, Coins},
    records::Records,
};
use tracing::info;

use crate::provider::RoundRobin;
use crate::{cache, database};

#[allow(clippy::module_name_repetitions)]
pub struct AppState {
    pub service: ProfileService,
}

impl AppState {
    pub async fn new() -> Self {
        let profile_records = env::var("PROFILE_RECORDS").map_or_else(
            |_| Records::default().records,
            |s| s.split(',').map(ToString::to_string).collect(),
        );

        let multicoin_chains: Vec<CoinType> = env::var("MULTICOIN_CHAINS").map_or_else(
            |_| Coins::default().coins,
            |s| {
                let numbers = s
                    .split(',')
                    .filter_map(|it| it.parse::<u64>().ok())
                    .collect::<Vec<_>>();

                numbers.iter().map(|num| CoinType::from(*num)).collect()
            },
        );

        let raw_rpc_urls: String = env::var("RPC_URL").expect("RPC_URL should've been set");
        let rpc_urls = raw_rpc_urls
            .split(',')
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        info!("Connecting to Redis...");

        let redis = database::setup().await.expect("Redis connection failed");

        info!("Connected to Redis");

        let provider = RoundRobin::new(rpc_urls);

        let opensea_api_key =
            env::var("OPENSEA_API_KEY").expect("OPENSEA_API_KEY should've been set");

        Self {
            service: ProfileService {
                cache: Box::new(cache::Redis::new(redis)),
                rpc: Box::new(provider),
                opensea_api_key,
                profile_records: Arc::from(profile_records),
                profile_chains: Arc::from(multicoin_chains),
            },
        }
    }
}
