use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod from_address;
pub mod from_name;
pub mod resolve_records;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Profile {
    pub name: String,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub display: Option<String>,
    pub records: HashMap<String, String>
}

#[derive(Debug)]
pub enum ProfileError {
    NotFound,
}
