use std::collections::BTreeMap;

use enstate_shared::{models::profile::Profile, utils::eip55::EIP55Address};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct SProfile {
    // Name
    pub name: String,
    // Ethereum Mainnet Address
    pub address: Option<String>,
    // Avatar URL
    pub avatar: Option<String>,
    // Preferred Capitalization of Name
    pub display: String,
    // Records
    pub records: BTreeMap<String, String>,
    // Addresses on different chains
    pub chains: BTreeMap<String, String>,
    // Unix Timestamp of date it was loaded
    pub fresh: i64,
    // Resolver the information was fetched from
    pub resolver: String,
    // Errors encountered while fetching & decoding
    pub errors: BTreeMap<String, String>,
}

impl From<Profile> for SProfile {
    fn from(profile: Profile) -> Self {
        Self {
            name: profile.name,
            address: profile.address.map(|a| a.to_string()),
            avatar: profile.avatar,
            display: profile.display,
            records: profile.records,
            chains: profile.chains,
            fresh: profile.fresh,
            resolver: profile.resolver.to_string(),
            errors: profile.errors,
        }
    }
}
