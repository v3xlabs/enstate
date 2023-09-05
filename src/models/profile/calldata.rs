use ethers::{
    types::{Bytes, H256},
    utils::hex::FromHex,
};

use crate::models::profile::Profile;

impl Profile {
    pub fn calldata_address(namehash: &H256) -> [u8; 36] {
        // combine
        // b"0x3b3b57de"
        // and
        // namehash.as_bytes()
        // such that the output is 0x3b3b57de + namehash.as_bytes()

        let bytes = Bytes::from_hex("3b3b57de").unwrap().to_vec();

        let mut result = [0u8; 36];

        result[..4].copy_from_slice(bytes.as_slice());
        result[4..].copy_from_slice(namehash.as_fixed_bytes());

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::{providers::namehash, utils::hex::FromHex};

    #[tokio::test]
    async fn test_calldata_address() {
        let namehash = namehash("eth");
        let calldata = Profile::calldata_address(&namehash);

        assert_eq!(
            calldata.to_vec(),
            Bytes::from_hex(
                "3b3b57de93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae"
            )
            .unwrap()
            .to_vec()
        );
    }

    #[tokio::test]
    async fn test_calldata_address_two() {
        let namehash = namehash("foo.eth");
        let calldata = Profile::calldata_address(&namehash);

        assert_eq!(
            calldata.to_vec(),
            Bytes::from_hex(
                "3b3b57dede9b09fd7c5f901e23a3f19fecc54828e9c848539801e86591bd9801b019f84f"
            )
            .unwrap()
            .to_vec()
        );
    }
}
