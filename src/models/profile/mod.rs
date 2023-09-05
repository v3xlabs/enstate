use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::collections::BTreeMap;
use utoipa::ToSchema;

pub mod from_address;
pub mod from_name;
pub mod resolve_avatar;
pub mod resolve_display;
pub mod resolve_records;
pub mod resolve_address;
pub mod calldata;
pub mod resolve_universal;

pub use resolve_records::default_records;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Profile {
    pub name: String,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub display: String,
    pub records: BTreeMap<String, String>,
    // Unix Timestamp of date it was loaded
    pub fresh: i64,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum ProfileError {
    NotFound,
}
