use std::convert::Infallible;

use ethers::{
    types::{Bytes, H256},
    utils::hex::FromHex,
};
use ethers_core::{abi::{Token, ParamType}, types::Address};

use crate::models::profile::Profile;

impl Profile {
    pub fn calldata_address(namehash: &H256) -> Vec<u8> {
        let fn_selector = Bytes::from_hex("3b3b57de").unwrap().to_vec();

        let data =
            ethers_core::abi::encode(&[Token::FixedBytes(namehash.as_fixed_bytes().to_vec())]);

        [fn_selector, data].concat()
    }

    pub fn decode_address(data: &[u8]) -> Result<Address, Infallible> {
        let address = ethers_core::abi::decode(&[ParamType::Address], data)
            .unwrap()
            .pop()
            .unwrap();

        Ok(address.into_address().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::{providers::namehash, utils::hex::FromHex};

    #[tokio::test]
    async fn test_calldata_address() {
        assert_eq!(
            hex::encode(Profile::calldata_address(&namehash("eth"))),
            "3b3b57de93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae"
        );

        assert_eq!(
            hex::encode(Profile::calldata_address(&namehash("foo.eth"))),
            "3b3b57dede9b09fd7c5f901e23a3f19fecc54828e9c848539801e86591bd9801b019f84f"
        );
    }
}
