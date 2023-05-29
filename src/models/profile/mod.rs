use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

pub mod from_name;
pub mod from_address;
pub mod resolve_owner;
pub mod resolve_avatar;
pub mod resolve_display;
pub mod resolve_records;
pub mod resolve_addresses;

pub use resolve_records::default_records;
pub use resolve_addresses::default_addresses;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Profile {
    pub name: String,
    pub resolver: String,
    pub owner: Option<String>,
    pub avatar: Option<String>,
    pub display: String,
    pub records: BTreeMap<String, String>,
    pub addresses: BTreeMap<String, String>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum ProfileError {
    NotFound,
}
