use std::str::FromStr;

use base32::Alphabet;
use cid::Cid;
use ethers_core::{
    abi::{ParamType, Token},
    types::H256,
};
use hex_literal::hex;
use tracing::info;
use utoipa::openapi::info;

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

    info!("contenthash: {:?}", contenthash);

    if contenthash.len() < 3 {
        return Err(ENSLookupError::ContentHashDecodeError);
    }

    info!("contenthash: {:?}", contenthash);

    let proto_code = contenthash[0];
    let other = contenthash[1];
    let value = &contenthash[2..];

    match proto_code {
        0xe3 => {
            // ipfs
            let value = cid::Cid::try_from(value)
                .map_err(|_| ENSLookupError::ContentHashDecodeError)?
                .to_string();
            Ok(format!("ipfs://{value}"))
        }
        0xe4 => {
            // swarm
            Err(ENSLookupError::Unsupported(
                "Swarm contenthash is not supported".to_string(),
            ))
        }
        0xe5 => {
            // ipns
            // let value = cid::Cid::try_from(value)
            //     .map_err(|_| ENSLookupError::ContentHashDecodeError)?
            //     .into_v1()
            //     .map_err(|_| ENSLookupError::ContentHashDecodeError)?
            //     .to_string();
            // // let value = String::from_utf8_lossy(
            // //     cid::Cid::try_from(value)
            // //         .map_err(|_| ENSLookupError::ContentHashDecodeError)?
            // //         .hash()
            // //         .digest()
            // // )
            // // .to_string();
            // Ok(format!("ipns://{value}"))
            Err(ENSLookupError::Unsupported("IPNS contenthash is not supported".to_string()))
        }
        // TODO: Add support for other contenthash types
        // onion
        // onion3
        // skynet
        // arweave
        other => Err(ENSLookupError::Unsupported(
            format!("Contenthash of this protoCode ({other}) is not supported").to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use ethers::providers::namehash;
    use hex_literal::hex;

    use crate::models::lookup::ENSLookup;

    #[test]
    fn test_calldata_address() {
        // assert_eq!(
        //     ENSLookup::ContentHash.calldata(&namehash("vitalik.eth")),
        //     hex!("3b3b57de93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae")
        // );

        // assert_eq!(
        //     ENSLookup::ContentHash.calldata(&namehash("vitalik.eth")),
        //     hex!("3b3b57dede9b09fd7c5f901e23a3f19fecc54828e9c848539801e86591bd9801b019f84f")
        // );
    }

    #[tokio::test]
    async fn test_decode() {
        assert_eq!(
            super::decode(&hex!(
                "e3010170122029f2d17be6139079dc48696d1f582a8530eb9805b561eda517e22a892c7e3f1f"
            ))
            .await
            .unwrap(),
            "ipfs://QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW1CuDgwxkD4".to_string()
        );
    }
}
