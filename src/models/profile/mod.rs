use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

pub mod error;
pub mod from_name;
pub mod from_address;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Profile {
    pub name: String,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub display: String,
    pub records: BTreeMap<String, String>,
    pub chains: BTreeMap<String, String>,
    // Unix Timestamp of date it was loaded
    pub fresh: i64,
    pub resolver: String,
}
