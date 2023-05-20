use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod from_address;
pub mod from_name;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Profile {
    pub name: String,
    pub address: Option<String>,
    pub avatar: Option<String>,
}

pub enum ProfileError {
    NotFound,
}
