use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::utils::eip55::EIP55Address;

pub mod error;
pub mod from_address;
pub mod from_name;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    // Name
    pub name: String,
    // Ethereum Mainnet Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<EIP55Address>,
    // Avatar URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    // Header URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    // Preferred Capitalization of Name
    pub display: String,
    // Records
    pub records: BTreeMap<String, String>,
    // Addresses on different chains
    pub chains: BTreeMap<String, String>,
    // Unix Timestamp of date it was loaded
    pub fresh: i64,
    // Resolver the information was fetched from
    pub resolver: EIP55Address,
    // Errors encountered while fetching & decoding
    pub errors: BTreeMap<String, String>,
}
