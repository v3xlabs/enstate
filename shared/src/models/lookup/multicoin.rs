use ethers_core::{
    abi::{ParamType, Token},
    types::H256,
};
use hex_literal::hex;

use crate::models::multicoin::cointype::coins::CoinType;

use super::{abi_decode_universal_ccip, ENSLookupError};

pub fn function_selector() -> [u8; 4] {
    hex!("f1cb7e06")
}

pub fn calldata(namehash: &H256, coin_type: &CoinType) -> Vec<u8> {
    let data = ethers_core::abi::encode(&[
        Token::FixedBytes(namehash.as_fixed_bytes().to_vec()),
        Token::Uint(coin_type.clone().into()),
    ]);

    [&function_selector() as &[u8], &data].concat()
}

pub async fn decode(data: &[u8], coin_type: &CoinType) -> Result<String, ENSLookupError> {
    let decoded_abi = abi_decode_universal_ccip(data, &[ParamType::Bytes])?;

    let Some(Token::Bytes(bytes)) = decoded_abi.first() else {
        return Err(ENSLookupError::AbiDecodeError);
    };

    if bytes.is_empty() {
        return Ok(String::new());
    }

    Ok(coin_type.decode(bytes.as_ref())?)
}
