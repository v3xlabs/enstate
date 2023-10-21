use std::{future::Future, pin::Pin};

use async_trait::async_trait;
use enstate_shared::cache::{CacheError, CacheLayer};

pub struct CloudflareKVCache {}

impl CloudflareKVCache {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl CacheLayer for CloudflareKVCache {
    async fn get(&self, key: &str) -> Result<String, CacheError> {
        Err(CacheError::Other("Not Implemented".to_string()))
    }

    async fn set(&self, key: &str, value: &str, expires: u32) -> Result<(), CacheError> {
        Ok(())
    }
}
