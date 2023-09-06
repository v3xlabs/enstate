use std::convert::Infallible;

use bs58::Alphabet;
use ethers::{
    types::{Bytes, H256},
    utils::hex::FromHex,
};
use ethers_core::{
    abi::{ParamType, Token},
    types::U256,
};
use tracing::info;

use crate::models::profile::Profile;

impl Profile {
    pub fn calldata_multicoin(namehash: &H256, coin_type: U256) -> Vec<u8> {
        let fn_selector = Bytes::from_hex("f1cb7e06").unwrap().to_vec();

        let data = ethers_core::abi::encode(&[
            Token::FixedBytes(namehash.as_fixed_bytes().to_vec()),
            Token::Uint(coin_type),
        ]);

        [fn_selector, data].concat()
    }

    pub fn decode_multicoin(data: &[u8], coin_type: U256) -> Result<String, Infallible> {
        let value = ethers_core::abi::decode(&[ParamType::Bytes], data)
            .unwrap()
            .pop()
            .unwrap();

        let value = value.into_bytes().unwrap();

        info!("value: {:?}", value);

        Ok(Self::decode_btc(value.as_slice()).unwrap())
    }

    pub fn decode_btc(data: &[u8]) -> Option<String> {
        // let value = bs58::encode(data).into_string();
        if !data.len().eq(&25) {
            return None;
        }

        // let v = hex::encode(data);

        let first_byte = data[0];
        let address = &data[1..21];

        let value = bs58::encode(address)
            .with_alphabet(Alphabet::BITCOIN)
            .into_string();

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calldata_btc() {
        let value = [
            118, 169, 20, 195, 11, 208, 69, 103, 38, 172, 25, 169, 223, 104, 3, 70, 169, 136, 230,
            107, 241, 207, 243, 136, 172,
        ];

        let x = Profile::decode_btc(&value);

        assert_eq!(x, Some("1JnJvEBykLcGHYxCZVWgDGDm7pkK3EBHwB".to_string()));
    }

    #[tokio::test]
    async fn test_btc() {
        assert_eq!(
            Profile::decode_btc(
                &hex::decode("76a91462e907b15cbf27d5425399ebf6f0fb50ebb88f1888ac").unwrap()
            ),
            Some("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string())
        );
    }
}
