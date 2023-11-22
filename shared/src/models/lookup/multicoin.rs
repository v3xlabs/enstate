use std::sync::Arc;

use async_trait::async_trait;
use ethers_core::{
    abi::{ParamType, Token},
    types::H256,
};
use hex_literal::hex;

use crate::models::multicoin::cointype::coins::CoinType;

use super::{abi_decode_universal_ccip, ENSLookup, ENSLookupError, LookupState};

pub struct Multicoin {
    pub coin_type: CoinType,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ENSLookup for Multicoin {
    fn calldata(&self, namehash: &H256) -> Vec<u8> {
        let fn_selector = hex!("f1cb7e06").to_vec();

        let data = ethers_core::abi::encode(&[
            Token::FixedBytes(namehash.as_fixed_bytes().to_vec()),
            Token::Uint(self.coin_type.clone().into()),
        ]);

        [fn_selector, data].concat()
    }

    async fn decode(&self, data: &[u8], _: Arc<LookupState>) -> Result<String, ENSLookupError> {
        let decoded_abi = abi_decode_universal_ccip(data, &[ParamType::Bytes])?;

        let value = decoded_abi
            .get(0)
            .ok_or(ENSLookupError::AbiDecodeError)?
            .clone()
            .into_bytes()
            .expect("token should be bytes");

        if value.is_empty() {
            // Empty field
            return Ok(String::new());
        }

        Ok(self.coin_type.decode(&value)?)
    }

    fn name(&self) -> String {
        format!("chains.{}", self.coin_type)
    }
}
