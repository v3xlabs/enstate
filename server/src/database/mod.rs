use std::env;

use anyhow::Result;
use redis::aio::ConnectionManager;

pub async fn setup() -> Result<ConnectionManager> {
    let redis = redis::Client::open(env::var("REDIS_URL").expect("REDIS_URL should've been set"))?;

    Ok(ConnectionManager::new(redis).await?)
}
