use ethers::addressbook::Address;

use super::{error::ProfileError, Profile, ProfileService};

impl ProfileService {
    pub async fn name_from_name_or_address(
        &self,
        name_or_address: &str,
        fresh: bool,
    ) -> Result<String, ProfileError> {
        if let Ok(address) = name_or_address.parse::<Address>() {
            return self.primary_from_address(address, fresh).await;
        }

        if !crate::patterns::test_domain(name_or_address) {
            return Err(ProfileError::NotFound);
        }

        Ok(name_or_address.to_lowercase())
    }

    pub async fn resolve_from_name_or_address(
        &self,
        name_or_address: &str,
        fresh: bool,
    ) -> Result<Profile, ProfileError> {
        let name = self
            .name_from_name_or_address(name_or_address, fresh)
            .await?;

        self.resolve_from_name(&name, fresh).await
    }
}
