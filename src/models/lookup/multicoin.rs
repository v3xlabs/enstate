use ethers_core::{
    abi::{ParamType, Token},
    types::H256,
};
use hex_literal::hex;

use crate::models::multicoin::cointype::coins::CoinType;

use super::{ENSLookup, ENSLookupError};

pub struct Multicoin {
    pub coin_type: CoinType,
}

impl ENSLookup for Multicoin {
    fn calldata(&self, namehash: &H256) -> Vec<u8> {
        let fn_selector = hex!("f1cb7e06").to_vec();

        let data = ethers_core::abi::encode(&[
            Token::FixedBytes(namehash.as_fixed_bytes().to_vec()),
            Token::Uint(self.coin_type.clone().into()),
        ]);

        [fn_selector, data].concat()
    }

    fn decode(&self, data: &[u8]) -> Result<String, ENSLookupError> {
        let decoded_abi = ethers_core::abi::decode(&[ParamType::Bytes], data)
            .map_err(|_| ENSLookupError::AbiDecodeError)?;
        let value = decoded_abi
            .get(0)
            .ok_or(ENSLookupError::AbiDecodeError)?
            .clone()
            .into_bytes();

        let value = value.unwrap();

        if value.is_empty() {
            // Empty field
            return Ok(String::new());
        }

        Ok(self.coin_type.decode(&value)?)
    }

    fn name(&self) -> String {
        format!("chains.{}", self.coin_type.to_string())
    }
}
