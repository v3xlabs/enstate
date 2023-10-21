use std::{
    fmt::Debug,
    future::Future,
    num::TryFromIntError,
    pin::Pin,
    sync::{Arc, Mutex, RwLock},
};

use async_trait::async_trait;
use enstate_shared::cache::{CacheError, CacheLayer};
use worker::{console_log, Env};
use worker_kv::KvStore;

// KvStore cannot be shared between threads because *mut u8 cannot be shared between threads safely within, the trait `Sync` is not implemented for `*mut u8` [E0277]
// Wrapper for KvStore such that it implements Sync
#[derive(Clone)]
pub struct SyncKvStore {
    v: Arc<RwLock<Arc<KvStore>>>,
}

unsafe impl Send for SyncKvStore {}
unsafe impl Sync for SyncKvStore {}

impl SyncKvStore {
    pub fn new(store: KvStore) -> Self {
        Self {
            v: Arc::new(RwLock::new(Arc::new(store))),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CloudflareKVCache {
    kv: SyncKvStore,
}

impl CloudflareKVCache {
    pub fn new(store: SyncKvStore) -> Self {
        Self { kv: store }
    }
}

#[async_trait]
impl CacheLayer for CloudflareKVCache {
    async fn get(&self, key: &str) -> Result<String, CacheError> {

    }

    async fn set(&self, key: &str, value: &str, expires: u32) -> Result<(), CacheError> {

    }
}

// #[async_trait]
// impl CacheLayer for CloudflareKVCache {
//     async fn get(&self, key: &str) -> Result<String, CacheError> {
//         let kv = self.v.v.lock().unwrap();

//         let x = kv.get(key)
//             .text()
//             .await
//             .map_err(|e| CacheError::Other(e.to_string()));
//         // let kv = self.kv.lock().unwrap();
//         // let kv = self.v.v.clone();
//         // let kv2 = kv.try_lock().unwrap();

//         // let x = kv2.get(key).text().await;

//         // match x {
//         //     Ok(x) => x.ok_or(CacheError::Other("No value found".to_string())),
//         //     Err(error) => Err(CacheError::Other(error.to_string())),
//         // }
//         Ok("".to_string())
//     }

//     async fn set(&self, key: &str, value: &str, expires: u32) -> Result<(), CacheError> {
//         // let mut kv = self.kv.lock().unwrap();

//         // let x: Result<(), _> = kv
//         //     .put(key, value)
//         //     .expire(expires)
//         //     .execute()
//         //     .await
//         //     .map_err(|e| CacheError::Other(e.to_string()));

//         // match x {
//         //     Ok(_) => Ok(()),
//         //     Err(error) => Err(error),
//         // }
//         Ok(())
//     }
// }
