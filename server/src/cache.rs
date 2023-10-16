use axum::async_trait;
use enstate_shared::cache::{CacheError, CacheLayer};
use redis::aio::ConnectionManager;

pub struct RedisCache {
    redis: ConnectionManager,
}

impl RedisCache {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }
}

#[async_trait]
impl CacheLayer for RedisCache {
    async fn get(&self, key: &str) -> Result<String, CacheError> {
        // TODO: Implement
        Ok("".to_string())
    }

    async fn set(&self, key: &str, value: &str, expires: u32) -> Result<(), CacheError> {
        // TODO: Implement
        Ok(())
    }
}
