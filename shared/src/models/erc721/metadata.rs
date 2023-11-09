use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    // attributes: Vec<Attribute>,
}
