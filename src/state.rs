use ethers::{
    prelude::rand::seq::SliceRandom,
    providers::{Http, Provider},
};
use ethers_ccip_read::CCIPReadMiddleware;
use redis::aio::ConnectionManager;
use std::env;
use tracing::info;

use crate::{database, provider::RoundRobinProvider};

#[allow(clippy::module_name_repetitions)]
pub struct AppState {
    pub redis: ConnectionManager,
    pub profile_records: Vec<String>,
    pub rpc_urls: Vec<String>,
    pub provider: RoundRobinProvider,
}

pub fn default_records() -> Vec<String> {
    vec![
        "url",
        "name",
        "email",
        "header",
        "location",
        "timezone",
        "language",
        "pronouns",
        "com.github",
        "org.matrix",
        "io.keybase",
        "description",
        "com.twitter",
        "com.discord",
        "social.bsky",
        "org.telegram",
        "social.mastodon",
    ]
    .into_iter()
    .map(ToString::to_string)
    .collect()
}

impl AppState {
    pub async fn new() -> Self {
        let profile_records = env::var("PROFILE_RECORDS")
            .ok()
            .map_or_else(default_records, |s| {
                s.split(',').map(ToString::to_string).collect::<Vec<_>>()
            });

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

        Self {
            redis,
            profile_records,
            rpc_urls,
            provider,
        }
    }
}
