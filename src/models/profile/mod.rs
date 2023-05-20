use crate::utils;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

pub mod from_address;
pub mod from_name;
pub mod resolve_records;

pub use resolve_records::default_records;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Profile {
    pub name: String,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub display: Option<String>,
    #[serde(serialize_with = "utils::serialize_skip_none")]
    pub records: BTreeMap<String, Option<String>>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum ProfileError {
    NotFound,
}
