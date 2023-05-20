use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

pub mod from_name;
pub mod from_address;
pub mod resolve_display;
pub mod resolve_records;

pub use resolve_records::default_records;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Profile {
    pub name: String,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub display: String,
    pub records: BTreeMap<String, String>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum ProfileError {
    NotFound,
}
