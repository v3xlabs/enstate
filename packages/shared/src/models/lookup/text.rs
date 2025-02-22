use ethers_core::{
    abi::{ParamType, Token},
    types::H256,
};
use hex_literal::hex;

use super::{abi_decode_universal_ccip, ENSLookupError};

pub fn function_selector() -> [u8; 4] {
    hex!("59d1d43c")
}

pub fn calldata(namehash: &H256, record: &str) -> Vec<u8> {
    let data = ethers_core::abi::encode(&[
        Token::FixedBytes(namehash.as_fixed_bytes().to_vec()),
        Token::String(record.to_string()),
    ]);

    [&function_selector() as &[u8], &data].concat()
}

pub async fn decode(data: &[u8]) -> Result<String, ENSLookupError> {
    let decoded_abi = abi_decode_universal_ccip(data, &[ParamType::String])?;

    let Some(Token::String(value)) = decoded_abi.first() else {
        return Err(ENSLookupError::AbiDecodeError);
    };

    Ok(value.to_string())
}
