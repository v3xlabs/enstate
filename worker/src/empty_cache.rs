use async_trait::async_trait;
use enstate_shared::cache::{CacheError, CacheLayer};

pub struct EmptyCache {}

impl EmptyCache {
    pub fn new() -> Self {
        Self {}
    }
}

unsafe impl Send for EmptyCache {}
unsafe impl Sync for EmptyCache {}

// #[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
impl CacheLayer for EmptyCache {
    async fn get(&self, key: &str) -> Result<String, CacheError> {
        Err(CacheError::Other("Not Found".to_string()))
    }

    async fn set(&self, key: &str, value: &str, expires: u32) -> Result<(), CacheError> {
        Ok(())
    }
}
