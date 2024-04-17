use std::collections::BTreeMap;

use enstate_shared::core::Profile;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ENSProfile {
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

impl From<Profile> for ENSProfile {
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
