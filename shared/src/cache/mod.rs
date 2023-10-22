use async_trait::async_trait;

#[derive(Debug)]
pub enum CacheError {
    Other(String),
}

#[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
pub trait CacheLayer: Send + Sync {
    async fn get(&self, key: &str) -> Result<String, CacheError>;
    async fn set(&self, key: &str, value: &str, expires: u32) -> Result<(), CacheError>;
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
pub trait CacheLayer: Send + Sync {
    async fn get(&self, key: &str) -> Result<String, CacheError>;
    async fn set(&self, key: &str, value: &str, expires: u32) -> Result<(), CacheError>;
}

