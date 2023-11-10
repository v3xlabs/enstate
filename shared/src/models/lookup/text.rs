use std::sync::Arc;

use ethers_core::{
    abi::{ParamType, Token},
    types::H256,
};
use hex_literal::hex;
use async_trait::async_trait;

use super::{ENSLookup, ENSLookupError, LookupState};
pub struct Text {
    key: String,
}

impl Text {
    pub const fn new(key: String) -> Self {
        Self { key }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ENSLookup for Text {
    fn calldata(&self, namehash: &H256) -> Vec<u8> {
        let fn_selector = hex!("59d1d43c").to_vec();

        let data = ethers_core::abi::encode(&[
            Token::FixedBytes(namehash.as_fixed_bytes().to_vec()),
            Token::String(self.key.to_string()),
        ]);

        [fn_selector, data].concat()
    }

    async fn decode(&self, data: &[u8], _: Arc<LookupState>) -> Result<String, ENSLookupError> {
        let decoded_abi = ethers_core::abi::decode(&[ParamType::String], data)
            .map_err(|_| ENSLookupError::AbiDecodeError)?;
        let value = decoded_abi.get(0).ok_or(ENSLookupError::AbiDecodeError)?;
        let value = value.to_string();

        Ok(value)
    }

    fn name(&self) -> String {
        format!("records.{}", self.key)
    }
}
