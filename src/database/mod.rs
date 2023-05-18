use anyhow::Result;
use redis::aio::ConnectionManager;
use std::env;

pub async fn setup() -> Result<ConnectionManager> {
    let redis = redis::Client::open(
        env::var("REDIS_URL").expect("REDIS_URL environment variable not found."),
    )?;

    Ok(ConnectionManager::new(redis).await?)
}
