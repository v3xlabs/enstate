use ethers_core::types::H256;
use thiserror::Error;

use super::multicoin::decoding::MulticoinDecoderError;

pub mod addr;
pub mod avatar;
pub mod multicoin;
pub mod text;

#[derive(Error, Debug)]
pub enum ENSLookupError {
    #[error("ABI error")]
    AbiDecodeError,

    #[error("MulticoinDecoderError: {0}")]
    MulticoinDecoder(#[from] MulticoinDecoderError),

    #[allow(dead_code)]
    #[error("Unsupported: {0}")]
    Unsupported(String),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

pub trait ENSLookup {
    fn calldata(&self, namehash: &H256) -> Vec<u8>;
    fn decode(&self, data: &[u8]) -> Result<String, ENSLookupError>;
    fn name(&self) -> String;
}
