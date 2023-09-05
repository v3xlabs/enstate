use std::convert::Infallible;

use ethers::{
    types::{Bytes, H256},
    utils::hex::FromHex,
};
use ethers_core::abi::{Token, ParamType};

use crate::models::profile::Profile;

impl Profile {
    pub fn calldata_text(namehash: &H256, key: &str) -> Vec<u8> {
        let fn_selector = Bytes::from_hex("59d1d43c").unwrap().to_vec();

        let data = ethers_core::abi::encode(&[
            Token::FixedBytes(namehash.as_fixed_bytes().to_vec()),
            Token::String(key.to_string()),
        ]);

        [fn_selector, data].concat()
    }

    pub fn decode_text(data: &[u8]) -> Result<String, Infallible> {
        let value = ethers_core::abi::decode(&[ParamType::String], data)
            .unwrap()
            .pop()
            .unwrap();

        Ok(value.into_string().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::providers::namehash;

    #[tokio::test]
    async fn test_calldata_text() {
        assert_eq!(
            hex::encode(Profile::calldata_text(&namehash("luc.eth"), "avatar")),
            "59d1d43ce1e7bcf2ca33c28a806ee265cfedf02fedf1b124ca73b2203ca80cc7c91a02ad000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000066176617461720000000000000000000000000000000000000000000000000000"
        );
    }
}
