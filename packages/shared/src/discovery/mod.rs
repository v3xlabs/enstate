use crate::core::{ENSService, Profile};
use async_trait::async_trait;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Discovery: Send + Sync {
    async fn discover_name(&self, profile: &Profile) -> Result<(), ()>;

    async fn query_search(&self, service: &ENSService, query: String) -> Result<Vec<Profile>, ()>;
}
