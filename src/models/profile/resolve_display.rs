use crate::models::profile::Profile;
use ethers::providers::{Http, Provider};
use ethers_ccip_read::CCIPReadMiddleware;

impl Profile {
    pub async fn resolve_display(name: &str, provider: CCIPReadMiddleware<Provider<Http>>) -> Option<String> {
        let Ok(Some(display)) = Self::resolve_record(name, "display", provider).await else { return None };

        (name.to_lowercase() == display.to_lowercase()).then_some(display)
    }
}
