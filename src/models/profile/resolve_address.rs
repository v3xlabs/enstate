use ethers::{providers::Middleware, types::H160};

use crate::{models::profile::Profile, state::AppState};

use super::ProfileError;

impl Profile {
    pub async fn resolve_address(name: &str, state: &AppState) -> Result<H160, ProfileError> {
        state.provider.resolve_name(name).await.map_err(|e| {
            println!("Error resolving name: {e:?}");

            ProfileError::NotFound
        })
    }
}
