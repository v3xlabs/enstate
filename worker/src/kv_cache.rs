use std::sync::Arc;

use async_trait::async_trait;
use enstate_shared::cache::{CacheError, CacheLayer};
use js_sys::{Function, Promise};
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use worker::Env;

use crate::getJS;

pub struct CloudflareKVCache {
    ctx: Arc<Env>,
}

impl CloudflareKVCache {
    pub fn new(ctx: Arc<Env>) -> Self {
        Self { ctx }
    }
}

unsafe impl Send for CloudflareKVCache {}
unsafe impl Sync for CloudflareKVCache {}

#[derive(Debug, Clone, Serialize)]
struct PutOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expirationTtl")]
    pub(crate) expiration_ttl: Option<u64>,
}

// #[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
impl CacheLayer for CloudflareKVCache {
    async fn get(&self, key: &str) -> Result<String, CacheError> {
        let kv_store = getJS(&self.ctx, "enstate-1").unwrap();
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
        let kv_store = getJS(&self.ctx, "enstate-1").unwrap();
        let put_function_value = getJS(&kv_store, "put").unwrap();

        let put_function = put_function_value.dyn_into::<Function>().unwrap();

        let options = PutOptions {
            expiration_ttl: Some(expires as u64),
        };

        let options_obj = serde_wasm_bindgen::to_value(&options).unwrap();

        let put_function_promise: Promise = put_function
            .call3(
                &kv_store,
                &JsValue::from_str(key),
                &JsValue::from_str(value),
                &options_obj,
            )
            .unwrap()
            .into();

        let put_function_result = JsFuture::from(put_function_promise);

        let _ = put_function_result
            .await
            .map_err(|_| CacheError::Other("Not Found".to_string()))?;

        Ok(())
    }
}
