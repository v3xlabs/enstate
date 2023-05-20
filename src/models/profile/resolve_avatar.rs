use ethers::providers::Middleware;

use crate::{models::profile::Profile, state::AppState};

impl Profile {
    pub async fn resolve_avatar(name: &str, state: &AppState) -> Option<String> {
        state
            .provider
            .resolve_avatar(name)
            .await
            .ok()
            .map(|result| result.to_string())
    }
}
