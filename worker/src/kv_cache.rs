use std::{
    num::TryFromIntError,
    sync::{Arc, Mutex}, future::Future, pin::Pin,
};

use async_trait::async_trait;
use enstate_shared::cache::{CacheError, CacheLayer};
use worker::{Env, console_log};
use worker_kv::KvStore;

#[derive(Clone, Debug)]
pub struct CloudflareKVCache {
    v: String,
}

impl CloudflareKVCache {
    pub fn new() -> Self {

        Self {
            v: "Hello World".to_string()
        }
    }
}

#[async_trait]
impl CacheLayer for CloudflareKVCache {
    async fn get(&self, key: &str) -> Result<String, CacheError> {
        // let kv = self.kv.lock().unwrap();
        // let x = kv.get(key).text().await;

        // match x {
        //     Ok(x) => x.ok_or(CacheError::Other("No value found".to_string())),
        //     Err(error) => Err(CacheError::Other(error.to_string())),
        // }
        Ok("".to_string())
    }

    async fn set(&self, key: &str, value: &str, expires: u32) -> Result<(), CacheError> {
        // let mut kv = self.kv.lock().unwrap();

        // let x: Result<(), _> = kv
        //     .put(key, value)
        //     .expire(expires)
        //     .execute()
        //     .await
        //     .map_err(|e| CacheError::Other(e.to_string()));

        // match x {
        //     Ok(_) => Ok(()),
        //     Err(error) => Err(error),
        // }
        Ok(())
    }
}
