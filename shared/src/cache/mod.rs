use async_trait::async_trait;

#[derive(Debug)]
pub enum CacheError {
    Other(String)
}

#[async_trait]
pub trait CacheLayer {
    async fn get(&self, key: &str) -> Result<String, CacheError>;
    async fn set(&self, key: &str, value: &str, expires: u32) -> Result<(), CacheError>;
}
