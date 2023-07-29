use ethers::providers::{Http, Middleware, Provider};
use ethers_ccip_read::CCIPReadMiddleware;

use crate::models::profile::Profile;

impl Profile {
    pub async fn resolve_avatar(name: &str, provider: CCIPReadMiddleware<Provider<Http>>) -> Option<String> {
        provider
            .resolve_avatar(name)
            .await
            .ok()
            .map(|result| result.to_string())
    }
}
