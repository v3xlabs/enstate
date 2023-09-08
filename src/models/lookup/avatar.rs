use super::*;
use ethers_core::abi::{ParamType, Token};
use hex_literal::hex;

pub struct Avatar {
    ipfs_gateway: String,
}

impl Avatar {}

impl ENSLookup for Avatar {
    fn calldata(&self, namehash: &H256) -> Vec<u8> {
        let fn_selector = hex!("59d1d43c").to_vec();

        let data = ethers_core::abi::encode(&[
            Token::FixedBytes(namehash.as_fixed_bytes().to_vec()),
            Token::String("avatar".to_string()),
        ]);

        [fn_selector, data].concat()
    }

    fn decode(&self, data: &[u8]) -> Result<Option<String>, ENSLookupError> {
        let decoded_abi = ethers_core::abi::decode(&[ParamType::String], data)
            .map_err(|_| ENSLookupError::AbiError)?;
        let value = decoded_abi.get(0).ok_or(ENSLookupError::AbiError)?;
        let value = value.to_string();

        Ok(Some(value))
    }
}
