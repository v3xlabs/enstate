use ethers::providers::Middleware;

use crate::{models::profile::Profile, state::AppState};

impl Profile {
    // TODO: invalid info for now
    pub async fn resolve_owner(name: &str, state: &AppState) -> Option<String> {
        state
            .provider
            .resolve_name(name)
            .await
            .ok()
            .map(|result| format!("{:?}", result))
    }
}
