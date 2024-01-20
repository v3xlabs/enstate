use ethers_core::{
    abi::{ParamType, Token},
    types::H256,
};
use hex_literal::hex;

use super::ENSLookupError;

pub fn function_selector() -> [u8; 4] {
    hex!("3b3b57de")
}

pub fn calldata(namehash: &H256) -> Vec<u8> {
    let data = ethers_core::abi::encode(&[Token::FixedBytes(namehash.as_fixed_bytes().to_vec())]);

    [&function_selector() as &[u8], &data].concat()
}

pub async fn decode(data: &[u8]) -> Result<String, ENSLookupError> {
    let decoded_abi = ethers_core::abi::decode(&[ParamType::Address], data)
        .map_err(|_| ENSLookupError::AbiDecodeError)?;

    let Some(Token::Address(address)) = decoded_abi.get(0) else {
        return Err(ENSLookupError::AbiDecodeError);
    };

    Ok(format!("{address:?}"))
}

#[cfg(test)]
mod tests {
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
