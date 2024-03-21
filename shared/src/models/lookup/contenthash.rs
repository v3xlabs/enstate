use ethers_core::{
    abi::{ParamType, Token},
    types::H256,
};
use hex_literal::hex;

use super::ENSLookupError;

pub fn function_selector() -> [u8; 4] {
    hex!("bc1c58d1")
}

pub fn calldata(namehash: &H256) -> Vec<u8> {
    let data = ethers_core::abi::encode(&[Token::FixedBytes(namehash.as_fixed_bytes().to_vec())]);

    [&function_selector() as &[u8], &data].concat()
}

pub async fn decode(data: &[u8]) -> Result<String, ENSLookupError> {
    let decoded_abi = ethers_core::abi::decode(&[ParamType::Bytes], data)
        .map_err(|_| ENSLookupError::AbiDecodeError)?;

    let Some(Token::Bytes(contenthash)) = decoded_abi.first() else {
        return Err(ENSLookupError::AbiDecodeError);
    };

    let proto_code = contenthash[0];
    let value = &contenthash[1..];

    match proto_code {
        0xe3 => {
            // ipfs
            Ok(format!("ipfs://{value:?}"))
        },
        0xe4 => {
            // swarm
            Err(ENSLookupError::Unsupported("Swarm contenthash is not supported".to_string()))
        },
        _ => {
            Err(ENSLookupError::Unsupported("Contenthash of this protoCode is not supported".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use ethers::providers::namehash;
    use hex_literal::hex;

    use crate::models::lookup::ENSLookup;

    #[test]
    fn test_calldata_address() {
        assert_eq!(
            ENSLookup::ContentHash.calldata(&namehash("vitalik.eth")),
            hex!("3b3b57de93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae")
        );

        assert_eq!(
            ENSLookup::ContentHash.calldata(&namehash("vitalik.eth")),
            hex!("3b3b57dede9b09fd7c5f901e23a3f19fecc54828e9c848539801e86591bd9801b019f84f")
        );
    }
}
