use std::sync::Arc;

use async_trait::async_trait;
use enstate_shared::cache::{CacheError, CacheLayer};
use js_sys::{Function, Promise, Reflect};
use serde::Serialize;
use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use worker::Env;

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

#[derive(Error, Debug)]
enum KVCacheJsError {
    #[error("JavaScript reflection error: {0}")]
    ReflectionError(String),

    #[error("JavaScript call error: {0}")]
    CallError(String),

    #[error("JavaScript cast error")]
    CastError,
}

fn call_error_from_jsvalue(value: JsValue) -> KVCacheJsError {
    KVCacheJsError::CallError(
        value
            .as_string()
            .unwrap_or_else(|| "unknown error".to_string()),
    )
}

fn get_js(target: &JsValue, name: &str) -> Result<JsValue, KVCacheJsError> {
    Reflect::get(target, &JsValue::from(name)).map_err(|err| {
        KVCacheJsError::ReflectionError(
            err.as_string()
                .unwrap_or_else(|| "unknown error".to_string()),
        )
    })
}

#[derive(Debug, Clone, Serialize)]
struct PutOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expirationTtl")]
    pub(crate) expiration_ttl: Option<u64>,
}

impl From<KVCacheJsError> for CacheError {
    fn from(value: KVCacheJsError) -> Self {
        CacheError::Other(value.to_string())
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
impl CacheLayer for CloudflareKVCache {
    async fn get(&self, key: &str) -> Result<String, CacheError> {
        let kv_store = get_js(&self.ctx, "enstate-1")?;
        let get_function_value = get_js(&kv_store, "get")?;

        let get_function = get_function_value
            .dyn_into::<Function>()
            .map_err(|_| KVCacheJsError::CastError)?;

        let options = JsValue::default();

        let get_function_promise: Promise = get_function
            .call2(&kv_store, &JsValue::from_str(key), &options)
            .map_err(call_error_from_jsvalue)?
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
        let kv_store = get_js(&self.ctx, "enstate-1")?;
        let put_function_value = get_js(&kv_store, "put")?;

        let put_function = put_function_value
            .dyn_into::<Function>()
            .map_err(|_| KVCacheJsError::CastError)?;

        let options = PutOptions {
            expiration_ttl: Some(expires as u64),
        };

        let options_obj =
            serde_wasm_bindgen::to_value(&options).map_err(|_| KVCacheJsError::CastError)?;

        let put_function_promise: Promise = put_function
            .call3(
                &kv_store,
                &JsValue::from_str(key),
                &JsValue::from_str(value),
                &options_obj,
            )
            .map_err(call_error_from_jsvalue)?
            .into();

        let put_function_result = JsFuture::from(put_function_promise);

        put_function_result
            .await
            .map(|_| ())
            .map_err(|_| CacheError::Other("Not Found".to_string()))?;

        Ok(())
    }
}
