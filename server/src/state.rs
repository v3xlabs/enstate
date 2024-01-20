use enstate_shared::cache::{CacheLayer, PassthroughCacheLayer};
use ethers_core::types::H160;
use std::env;
use std::sync::Arc;

use enstate_shared::core::ENSService;
use enstate_shared::models::{
    multicoin::cointype::{coins::CoinType, Coins},
    records::Records,
};
use tracing::{info, warn};

use crate::provider::RoundRobin;
use crate::{cache, database};

#[allow(clippy::module_name_repetitions)]
pub struct AppState {
    pub service: ENSService,
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

        let cache = database::setup().await.map_or_else(
            |_| {
                warn!("failed to connect to redis, using no cache");

                Box::new(PassthroughCacheLayer {}) as Box<dyn CacheLayer>
            },
            |redis| {
                info!("Connected to Redis");

                Box::new(cache::Redis::new(redis)) as Box<dyn CacheLayer>
            },
        );

        let provider = RoundRobin::new(rpc_urls);

        let opensea_api_key =
            env::var("OPENSEA_API_KEY").expect("OPENSEA_API_KEY should've been set");

        let universal_resolver = env::var("UNIVERSAL_RESOLVER")
            .expect("UNIVERSAL_RESOLVER should've been set")
            .parse::<H160>()
            .expect("UNIVERSAL_RESOLVER should be a valid address");

        Self {
            service: ENSService {
                cache,
                rpc: Box::new(provider),
                opensea_api_key,
                profile_records: Arc::from(profile_records),
                profile_chains: Arc::from(multicoin_chains),
                universal_resolver,
            },
        }
    }
}
