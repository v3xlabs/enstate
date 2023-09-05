use std::convert::Infallible;

use ethers::{
    types::{Bytes, H256},
    utils::hex::FromHex,
};
use ethers_core::abi::Token;

use crate::models::profile::Profile;

impl Profile {
    pub fn calldata_avatar(namehash: &H256) -> Vec<u8> {
        Self::calldata_text(namehash, "avatar")
    }

    pub fn decode_avatar(data: &[u8]) -> Result<String, Infallible> {
        // TODO: Add ipfs & arweave support
        Self::decode_text(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::providers::namehash;

    #[tokio::test]
    async fn test_calldata_avatar() {
        assert_eq!(
            hex::encode(Profile::calldata_avatar(&namehash("luc.eth"))),
            "59d1d43ce1e7bcf2ca33c28a806ee265cfedf02fedf1b124ca73b2203ca80cc7c91a02ad000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000066176617461720000000000000000000000000000000000000000000000000000"
        );
    }
}
