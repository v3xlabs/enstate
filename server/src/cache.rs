use std::num::TryFromIntError;

use axum::async_trait;
use enstate_shared::cache::{CacheError, CacheLayer};
use redis::{aio::ConnectionManager, AsyncCommands};

pub struct RedisCache {
    redis: ConnectionManager,
}

impl RedisCache {
    pub const fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }
}

#[async_trait]
impl CacheLayer for RedisCache {
    async fn get(&self, key: &str) -> Result<String, CacheError> {
        let mut redis = self.redis.clone();

        let x: Result<String, _> = redis.get(key).await;

        match x {
            Ok(x) => Ok(x),
            Err(error) => Err(CacheError::Other(error.to_string())),
        }
    }

    async fn set(&self, key: &str, value: &str, expires: u32) -> Result<(), CacheError> {
        let mut redis = self.redis.clone();

        let x: Result<(), _> = redis
            .set_ex(
                key,
                value,
                expires
                    .try_into()
                    .map_err(|x: TryFromIntError| CacheError::Other(x.to_string()))?,
            )
            .await;

        match x {
            Ok(_) => Ok(()),
            Err(error) => Err(CacheError::Other(error.to_string())),
        }
    }
}
