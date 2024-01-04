use ethers::addressbook::Address;

use super::{error::ProfileError, Profile, ProfileService};

impl ProfileService {
    pub async fn resolve_from_name_or_address(
        &self,
        name_or_address: &str,
        fresh: bool,
    ) -> Result<Profile, ProfileError> {
        if let Ok(address) = name_or_address.parse::<Address>() {
            return self.resolve_from_address(address, fresh).await;
        }

        if !crate::patterns::test_domain(name_or_address) {
            return Err(ProfileError::NotFound);
        }

        self.resolve_from_name(&name_or_address.to_lowercase(), fresh)
            .await
    }
}
