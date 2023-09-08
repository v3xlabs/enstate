use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

pub mod error;
pub mod from_address;
pub mod from_name;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Profile {
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
