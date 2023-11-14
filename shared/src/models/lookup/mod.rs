use std::fmt::Display;
use std::sync::Arc;

use async_trait::async_trait;
use ethers::providers::{Http, Provider};
use ethers_core::types::H256;
use thiserror::Error;

use crate::models::eip155::EIP155Error;

use super::multicoin::decoding::MulticoinDecoderError;

pub mod addr;
pub mod image;
pub mod multicoin;
pub mod text;

#[derive(Error, Debug)]
pub enum ENSLookupError {
    #[error("ABI error")]
    AbiDecodeError,

    #[error("MulticoinDecoderError: {0}")]
    MulticoinDecoder(#[from] MulticoinDecoderError),

    #[error("Unsupported: {0}")]
    Unsupported(String),

    #[error("EIP155: {0}")]
    EIP155Error(#[from] EIP155Error),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ENSLookup {
    fn calldata(&self, namehash: &H256) -> Vec<u8>;
    async fn decode(&self, data: &[u8], state: Arc<LookupState>) -> Result<String, ENSLookupError>;
    fn name(&self) -> String;

    fn to_boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl Display for dyn ENSLookup + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "ENSLookup({})", self.name())
    }
}

pub struct LookupState {
    pub rpc: Arc<Provider<Http>>,
    pub opensea_api_key: String,
}
