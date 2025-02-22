use std::fmt::Debug;

use async_trait::async_trait;

#[derive(Debug)]
pub enum CacheError {
    Other(String),
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait CacheLayer: Send + Sync {
    async fn get(&self, key: &str) -> Result<String, CacheError>;
    async fn set(&self, key: &str, value: &str, expires: u32) -> Result<(), CacheError>;
}

pub struct PassthroughCacheLayer {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl CacheLayer for PassthroughCacheLayer {
    async fn get(&self, _key: &str) -> Result<String, CacheError> {
        Err(CacheError::Other("".to_string()))
    }

    async fn set(&self, _key: &str, _value: &str, _expires: u32) -> Result<(), CacheError> {
        Ok(())
    }
}
