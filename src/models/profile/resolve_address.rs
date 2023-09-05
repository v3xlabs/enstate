use ethers::{
    providers::{Http, Middleware, Provider},
    types::H160,
};
use ethers_ccip_read::CCIPReadMiddleware;

use crate::models::profile::Profile;

use super::ProfileError;

impl Profile {
    pub async fn resolve_address(
        name: &str,
        provider: CCIPReadMiddleware<Provider<Http>>,
    ) -> Result<H160, ProfileError> {
        provider.resolve_name(name).await.map_err(|e| {
            println!("Error resolving name: {e:?}");

            ProfileError::NotFound
        })
    }
}
