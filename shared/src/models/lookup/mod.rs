use std::fmt::Display;
use std::sync::Arc;

use async_trait::async_trait;
use ethers::providers::{Http, Provider};
use ethers_ccip_read::CCIPReadMiddleware;
use ethers_core::abi;
use ethers_core::abi::Token;
use ethers_core::types::H256;
use lazy_static::lazy_static;
use thiserror::Error;

use crate::models::eip155::EIP155Error;

use super::multicoin::decoding::MulticoinDecoderError;

pub mod addr;
pub mod image;
pub mod multicoin;
pub mod text;

#[derive(Error, Debug)]
pub enum ENSLookupError {
    #[error("ABI decode error")]
    AbiDecodeError,

    #[error("ABI error: {0}")]
    AbiError(#[from] abi::Error),

    #[error("MulticoinDecoderError: {0}")]
    MulticoinDecoder(#[from] MulticoinDecoderError),

    #[error("Unsupported: {0}")]
    Unsupported(String),

    #[error("EIP155: {0}")]
    EIP155Error(#[from] EIP155Error),

    #[error("CCIP resolution error ({}): {}", status, message)]
    CCIPError { status: u16, message: String },
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
    pub rpc: Arc<CCIPReadMiddleware<Provider<Http>>>,
    pub opensea_api_key: String,
}

lazy_static! {
    static ref UNIVERSAL_RESOLVER_CCIP_ERROR: &'static [abi::ParamType; 1] =
        Box::leak(Box::new([abi::ParamType::Array(Box::from(
            abi::ParamType::Tuple(vec![abi::ParamType::Uint(16), abi::ParamType::String]),
        ))]));
}

pub fn abi_decode_universal_ccip(
    data: &[u8],
    types: &[abi::ParamType],
) -> Result<Vec<Token>, ENSLookupError> {
    abi::decode(types, data).map_err(|err| {
        if data.len() < 4 {
            return ENSLookupError::AbiError(err);
        }

        let results = abi::decode(*UNIVERSAL_RESOLVER_CCIP_ERROR, &data[4..]);

        let Ok(results) = results else {
            return ENSLookupError::AbiError(err);
        };

        let Some(Token::Array(errors)) = results.get(0) else {
            return ENSLookupError::AbiError(err);
        };

        let Some(Token::Tuple(tuple)) = errors.get(0) else {
            return ENSLookupError::AbiError(err);
        };

        let (Some(Token::Uint(status)), Some(Token::String(message))) =
            (tuple.get(0), tuple.get(1))
        else {
            return ENSLookupError::AbiError(err);
        };

        ENSLookupError::CCIPError {
            status: status.as_u32() as u16,
            message: message.to_string(),
        }
    })
}
