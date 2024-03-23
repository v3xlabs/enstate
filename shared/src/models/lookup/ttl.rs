use ethers_core::{
    abi::{ParamType, Token},
    types::H256,
};
use hex_literal::hex;

use super::ENSLookupError;

pub fn function_selector() -> [u8; 4] {
    hex!("16a25cbd")
}

pub fn calldata(namehash: &H256) -> Vec<u8> {
    let data = ethers_core::abi::encode(&[Token::FixedBytes(namehash.as_fixed_bytes().to_vec())]);

    [&function_selector() as &[u8], &data].concat()
}

pub async fn decode(data: &[u8]) -> Result<String, ENSLookupError> {
    dbg!(hex::encode(data));
    let decoded_abi = ethers_core::abi::decode(&[ParamType::Uint(64)], data)
        .map_err(|_| ENSLookupError::AbiDecodeError)?;

    let Some(Token::Uint(ttl)) = decoded_abi.first() else {
        return Err(ENSLookupError::AbiDecodeError);
    };

    Ok(format!("{}", ttl.as_u64()))
}
