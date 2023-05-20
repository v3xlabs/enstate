use ethers::providers::{Http, Provider};
use ethers_ccip_read::CCIPReadMiddleware;
use redis::aio::ConnectionManager;
use std::env;

use crate::{database, models::profile::default_records};

#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct AppState {
    pub redis: ConnectionManager,
    pub provider: CCIPReadMiddleware<Provider<Http>>,
    pub profile_records: Vec<String>,
    pub fallback_provider: Provider<Http>,
}

impl AppState {
    pub async fn new() -> Self {
        let profile_records = env::var("PROFILE_RECORDS")
            .ok()
            .map_or_else(default_records, |s| {
                s.split(',').map(ToString::to_string).collect::<Vec<_>>()
            });

        let rpc_url = env::var("RPC_URL").expect("RPC_URL environment variable not found.");

        let redis = database::setup().await.expect("Failed to connect to Redis");
        let fallback_provider = Provider::<Http>::try_from(rpc_url).unwrap();
        let provider: CCIPReadMiddleware<Provider<Http>> =
            CCIPReadMiddleware::new(fallback_provider.clone());

        Self {
            redis,
            provider,
            profile_records,
            fallback_provider,
        }
    }
}
