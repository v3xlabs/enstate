use std::collections::BTreeMap;

use enstate_shared::core::Profile;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct ENSProfile {
    // Name
    #[schema(example = "vitalik.eth")]
    pub name: String,
    // Ethereum Mainnet Address
    #[schema(example = "0x225f137127d9067788314bc7fcc1f36746a3c3B5")]
    pub address: Option<String>,
    // Avatar URL
    #[schema(example = "https://cloudflare-ipfs.com/ipfs/bafkreifnrjhkl7ccr2ifwn2n7ap6dh2way25a6w5x2szegvj5pt4b5nvfu")]
    pub avatar: Option<String>,
    // Preferred Capitalization of Name
    #[schema(example = "LuC.eTh")]
    pub display: String,
    // Records
    pub records: BTreeMap<String, String>,
    // Addresses on different chains
    pub chains: BTreeMap<String, String>,
    // Unix Timestamp of date it was loaded
    #[schema(example = "1713363899484")]
    pub fresh: i64,
    // Resolver the information was fetched from
    #[schema(example = "0x4976fb03C32e5B8cfe2b6cCB31c09Ba78EBaBa41")]
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
