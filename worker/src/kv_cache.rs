use std::sync::Arc;

use async_trait::async_trait;
use enstate_shared::cache::{CacheError, CacheLayer};
use js_sys::{Function, Promise};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use worker::{console_log, Env, RouteContext};
use worker_kv::KvStore;

use crate::getJS;

pub struct CloudflareKVCache {
    ctx: Arc<RouteContext<()>>,
}

impl CloudflareKVCache {
    pub fn new(ctx: Arc<RouteContext<()>>) -> Self {
        Self { ctx }
    }
}

unsafe impl Send for CloudflareKVCache {}
unsafe impl Sync for CloudflareKVCache {}

#[async_trait(?Send)]
impl CacheLayer for CloudflareKVCache {
    async fn get(&self, key: &str) -> Result<String, CacheError> {
        let kv_store = getJS(&self.ctx.env, "enstate-1").unwrap();
        let get_function_value = getJS(&kv_store, "get").unwrap();

        let get_function = get_function_value.dyn_into::<Function>().unwrap();

        let options = JsValue::default();

        let get_function_promise: Promise = get_function
            .call2(&kv_store, &JsValue::from_str(key), &options)
            .unwrap()
            .into();

        let get_function_result = JsFuture::from(get_function_promise);

        let result = get_function_result
            .await
            .map_err(|_| CacheError::Other("Not Found".to_string()))?;

        result
            .as_string()
            .ok_or(CacheError::Other("Not Found".to_string()))
    }

    async fn set(&self, key: &str, value: &str, expires: u32) -> Result<(), CacheError> {
        let kv_store = getJS(&self.ctx.env, "enstate-1").unwrap();
        let put_function_value = getJS(&kv_store, "put").unwrap();

        let put_function = put_function_value.dyn_into::<Function>().unwrap();

        let options = JsValue::default();

        let put_function_promise: Promise = put_function
            .call3(&kv_store, &JsValue::from_str(key), &JsValue::from_str(value), &options)
            .unwrap()
            .into();

        let put_function_result = JsFuture::from(put_function_promise);

        let _ = put_function_result
            .await
            .map_err(|_| CacheError::Other("Not Found".to_string()))?;

        Ok(())
    }
}
