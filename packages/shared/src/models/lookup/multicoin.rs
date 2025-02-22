use ethers_core::{
    abi::{ParamType, Token},
    types::H256,
};
use hex_literal::hex;

use crate::models::multicoin::cointype::coins::CoinType;
use crate::models::multicoin::cointype::evm::ChainId;
use crate::models::multicoin::cointype::slip44::SLIP44;

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
    // hell
    let decoded_abi = abi_decode_universal_ccip(data, &[ParamType::Bytes]).or_else(|err| {
        if coin_type != &CoinType::Slip44(SLIP44::Ethereum)
            && coin_type != &CoinType::Evm(ChainId::Ethereum)
        {
            return Err(err);
        }

        // hell^2
        abi_decode_universal_ccip(data, &[ParamType::Address]).map(|mut addr| {
            vec![Token::Bytes(
                addr.remove(0)
                    .into_address()
                    .expect("token should be an address")
                    .as_bytes()
                    .into(),
            )]
        })
    })?;

    let Some(Token::Bytes(bytes)) = decoded_abi.first() else {
        return Err(ENSLookupError::AbiDecodeError);
    };

    // please nobody take inspiration from this function
    if bytes.is_empty() || bytes.iter().all(|&b| b == 0) {
        return Ok(String::new());
    }

    Ok(coin_type.decode(bytes.as_ref())?)
}
