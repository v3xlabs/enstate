use std::sync::Arc;

use async_trait::async_trait;
use ethers_core::{
    abi::{ParamType, Token},
    types::H256,
};
use hex_literal::hex;

use super::{ENSLookup, ENSLookupError, LookupState};

pub struct Addr {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ENSLookup for Addr {
    fn calldata(&self, namehash: &H256) -> Vec<u8> {
        let fn_selector = hex!("3b3b57de").to_vec();

        let data =
            ethers_core::abi::encode(&[Token::FixedBytes(namehash.as_fixed_bytes().to_vec())]);

        [fn_selector, data].concat()
    }

    async fn decode(&self, data: &[u8], _: Arc<LookupState>) -> Result<String, ENSLookupError> {
        let decoded_abi = ethers_core::abi::decode(&[ParamType::Address], data)
            .map_err(|_| ENSLookupError::AbiDecodeError)?;

        let Some(Token::Address(address)) = decoded_abi.get(0) else {
            return Err(ENSLookupError::AbiDecodeError);
        };

        Ok(format!("{address:?}"))
    }

    fn name(&self) -> String {
        "addr".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::providers::namehash;
    use hex_literal::hex;

    #[test]
    fn test_calldata_address() {
        assert_eq!(
            Addr {}.calldata(&namehash("eth")),
            hex!("3b3b57de93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae")
        );

        assert_eq!(
            Addr {}.calldata(&namehash("foo.eth")),
            hex!("3b3b57dede9b09fd7c5f901e23a3f19fecc54828e9c848539801e86591bd9801b019f84f")
        );
    }
}
