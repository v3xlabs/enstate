use std::env;
use std::sync::Arc;

use enstate_shared::cache::{CacheLayer, PassthroughCacheLayer};
use enstate_shared::core::ENSService;
use enstate_shared::models::{
    multicoin::cointype::{coins::CoinType, Coins},
    records::Records,
};
use ethers_core::types::H160;
use tracing::{info, warn};
use url::Url;

use crate::discovery::engine::DiscoveryEngine;
use crate::http::RateLimiter;
use crate::provider::RoundRobin;
use crate::telemetry::metrics::Metrics;
use crate::{cache, database};

#[allow(clippy::module_name_repetitions)]
pub struct AppState {
    pub service: ENSService,
    pub metrics: Metrics,
    pub rate_limiter: RateLimiter,
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

        let ipfs_gateway = env::var("IPFS_GATEWAY").map_or_else(
            |_| "https://ipfs.io/ipfs/".to_string(),
            |ipfs_gateway| Url::parse(&ipfs_gateway).unwrap().to_string(),
        );

        let arweave_gateway = env::var("AR_GATEWAY").map_or_else(
            |_| "https://arweave.net/".to_string(),
            |arweave_gateway| Url::parse(&arweave_gateway).unwrap().to_string(),
        );

        let universal_resolver = env::var("UNIVERSAL_RESOLVER")
            .expect("UNIVERSAL_RESOLVER should've been set")
            .parse::<H160>()
            .expect("UNIVERSAL_RESOLVER should be a valid address");

        let max_bulk_size =
            env::var("MAX_BULK_SIZE").map_or(10, |bulk_size| bulk_size.parse().unwrap());

        let cache_ttl =
            env::var("PROFILE_CACHE_TTL").map_or(Some(600), |cache_ttl| cache_ttl.parse().ok());

        let discovery_engine = DiscoveryEngine::new("http://localhost:8123", "admin", "admin");

        discovery_engine.create_table_if_not_exists().await;

        Self {
            rate_limiter: RateLimiter::new(),
            service: ENSService {
                discovery: Some(Box::new(discovery_engine)),
                cache,
                rpc: Box::new(provider),
                opensea_api_key,
                ipfs_gateway,
                arweave_gateway,
                max_bulk_size,
                cache_ttl,
                profile_records: Arc::from(profile_records),
                profile_chains: Arc::from(multicoin_chains),
                universal_resolver,
            },
            metrics: Metrics::new(),
        }
    }
}
