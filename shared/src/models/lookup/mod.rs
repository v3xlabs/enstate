use std::hash::Hash;
use std::sync::Arc;

use ethers_core::abi;
use ethers_core::abi::Token;
use ethers_core::types::H256;
use lazy_static::lazy_static;
use thiserror::Error;
use tracing::instrument;

use crate::core::CCIPProvider;
use crate::models::eip155::EIP155Error;
use crate::models::multicoin::cointype::coins::CoinType;

use super::multicoin::decoding::MulticoinDecoderError;

pub mod addr;
pub mod image;
pub mod multicoin;
pub mod text;
pub mod contenthash;

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

    #[error("ContentHashDecodeError")]
    ContentHashDecodeError,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ENSLookup {
    Addr,
    Text(String),
    StaticText(&'static str),
    Image(String),
    StaticImage(&'static str),
    Multicoin(CoinType),
    ContentHash,
}

impl ENSLookup {
    pub fn function_selector(&self) -> [u8; 4] {
        match self {
            ENSLookup::Addr => addr::function_selector(),
            ENSLookup::Text(_) => text::function_selector(),
            ENSLookup::StaticText(_) => text::function_selector(),
            ENSLookup::Image(_) => image::function_selector(),
            ENSLookup::StaticImage(_) => image::function_selector(),
            ENSLookup::Multicoin(_) => multicoin::function_selector(),
            ENSLookup::ContentHash => contenthash::function_selector(),
        }
    }

    pub fn calldata(&self, namehash: &H256) -> Vec<u8> {
        match self {
            ENSLookup::Addr => addr::calldata(namehash),
            ENSLookup::Text(record) => text::calldata(namehash, record),
            ENSLookup::StaticText(record) => text::calldata(namehash, record),
            ENSLookup::Image(record) => image::calldata(namehash, record),
            ENSLookup::StaticImage(record) => image::calldata(namehash, record),
            ENSLookup::Multicoin(coin_type) => multicoin::calldata(namehash, coin_type),
            ENSLookup::ContentHash => contenthash::calldata(namehash),
        }
    }

    #[instrument]
    pub async fn decode(
        &self,
        data: &[u8],
        lookup_state: &LookupState,
    ) -> Result<String, ENSLookupError> {
        match self {
            ENSLookup::Addr => addr::decode(data).await,
            ENSLookup::Text(_) => text::decode(data).await,
            ENSLookup::StaticText(_) => text::decode(data).await,
            ENSLookup::Image(_) => image::decode(data, lookup_state).await,
            ENSLookup::StaticImage(_) => image::decode(data, lookup_state).await,
            ENSLookup::Multicoin(coin_type) => multicoin::decode(data, coin_type).await,
            ENSLookup::ContentHash => contenthash::decode(data).await,
        }
    }

    pub fn name(&self) -> String {
        match self {
            ENSLookup::Addr => "addr".to_string(),
            ENSLookup::Text(record) => format!("records.{}", record),
            ENSLookup::StaticText(record) => format!("records.{}", record),
            ENSLookup::Image(record) => format!("image.{}", record),
            ENSLookup::StaticImage(record) => format!("image.{}", record),
            ENSLookup::Multicoin(coin_type) => format!("chains.{}", coin_type),
            ENSLookup::ContentHash => "contenthash".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct LookupState {
    pub rpc: Arc<CCIPProvider>,
    pub opensea_api_key: String,
    pub ipfs_gateway: String,
    pub arweave_gateway: String,
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

        let Some(Token::Array(errors)) = results.first() else {
            return ENSLookupError::AbiError(err);
        };

        let Some(Token::Tuple(tuple)) = errors.first() else {
            return ENSLookupError::AbiError(err);
        };

        let (Some(Token::Uint(status)), Some(Token::String(message))) =
            (tuple.first(), tuple.get(1))
        else {
            return ENSLookupError::AbiError(err);
        };

        ENSLookupError::CCIPError {
            status: status.as_u32() as u16,
            message: message.to_string(),
        }
    })
}
