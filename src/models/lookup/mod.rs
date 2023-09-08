use ethers_core::types::H256;
use thiserror::Error;

pub mod addr;
pub mod text;
pub mod avatar;

#[derive(Error, Debug)]
pub enum ENSLookupError {
    #[error("ABI error")]
    AbiError,
    #[error("Invalid payload: {0}")]
    InvalidPayload(String),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

pub trait ENSLookup {
    fn calldata(&self, namehash: &H256) -> Vec<u8>;
    fn decode(&self, data: &[u8]) -> Result<Option<String>, ENSLookupError>;
}
