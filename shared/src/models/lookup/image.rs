use std::sync::Arc;

use async_trait::async_trait;
use ethers_core::types::U256;
use ethers_core::{
    abi::{ParamType, Token},
    types::H256,
};
use hex_literal::hex;
use lazy_static::lazy_static;
use thiserror::Error;
use tracing::info;

use crate::models::eip155::{resolve_eip155, EIP155ContractType};
use crate::models::multicoin::cointype::evm::ChainId;

use super::{abi_decode_universal_ccip, ENSLookup, ENSLookupError, LookupState};

pub struct Image {
    pub ipfs_gateway: String,
    pub name: String,
    pub record: String,
}

lazy_static! {
    static ref IPFS_REGEX: regex::Regex =
        regex::Regex::new(r"ipfs://([0-9a-zA-Z]+)").expect("should be a valid regex");
    static ref EIP155_REGEX: regex::Regex =
        regex::Regex::new(r"eip155:([0-9]+)/(erc1155|erc721):0x([0-9a-fA-F]{40})/([0-9]+)")
            .expect("should be a valid regex");
}

#[derive(Error, Debug)]
enum ImageLookupError {
    #[error("Format error: {0}")]
    FormatError(String),
}

impl From<ImageLookupError> for ENSLookupError {
    fn from(error: ImageLookupError) -> Self {
        ENSLookupError::Unsupported(error.to_string())
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ENSLookup for Image {
    fn calldata(&self, namehash: &H256) -> Vec<u8> {
        let fn_selector = hex!("59d1d43c").to_vec();

        let data = ethers_core::abi::encode(&[
            Token::FixedBytes(namehash.as_fixed_bytes().to_vec()),
            Token::String(self.record.clone()),
        ]);

        [fn_selector, data].concat()
    }

    async fn decode(&self, data: &[u8], state: Arc<LookupState>) -> Result<String, ENSLookupError> {
        let decoded_abi = abi_decode_universal_ccip(data, &[ParamType::String])?;
        let value = decoded_abi.get(0).ok_or(ENSLookupError::AbiDecodeError)?;
        let value = value.to_string();

        let opensea_api_key = state.opensea_api_key.clone();

        if let Some(captures) = IPFS_REGEX.captures(&value) {
            let hash = captures.get(1).unwrap().as_str();

            return Ok(format!("{}{hash}", self.ipfs_gateway));
        }

        let Some(captures) = EIP155_REGEX.captures(&value) else {
            return Ok(value);
        };

        let chain_id = captures.get(1).unwrap().as_str();
        let contract_type = captures.get(2).unwrap().as_str();
        let contract_address = captures.get(3).unwrap().as_str();
        let token_id = captures.get(4).unwrap().as_str();

        let chain_id = chain_id
            .parse::<u64>()
            .map_err(|err| ImageLookupError::FormatError(err.to_string()))?;

        let token_id = U256::from_dec_str(token_id)
            .map_err(|err| ImageLookupError::FormatError(err.to_string()))?;

        let contract_type = match contract_type {
            "erc721" => EIP155ContractType::ERC721,
            "erc1155" => EIP155ContractType::ERC1155,
            _ => {
                return Err(
                    ImageLookupError::FormatError("invalid contract type".to_string()).into(),
                )
            }
        };

        info!(
            "Encountered Avatar: {chain_id} {contract_type} {contract_address} {token_id}",
            chain_id = chain_id,
            contract_type = contract_type.as_str(),
            contract_address = contract_address,
            token_id = token_id
        );

        let resolved_uri = resolve_eip155(
            ChainId::from(chain_id),
            contract_type,
            contract_address,
            token_id,
            &state.rpc,
            &opensea_api_key,
        )
        .await?;

        // TODO: Remove naive approach
        return Ok(resolved_uri);
    }

    fn name(&self) -> String {
        self.record.clone()
    }
}

#[cfg(test)]
mod tests {
    use ethers::providers::namehash;

    use super::*;

    #[test]
    fn test_calldata_avatar() {
        assert_eq!(
            Image {
                ipfs_gateway: "https://ipfs.io/ipfs/".to_string(),
                name: "luc.eth".to_string(),
                record: "avatar".to_string()
            }.calldata(&namehash("luc.eth")),
            hex_literal::hex!("59d1d43ce1e7bcf2ca33c28a806ee265cfedf02fedf1b124ca73b2203ca80cc7c91a02ad000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000066176617461720000000000000000000000000000000000000000000000000000")
        );
    }

    #[test]
    fn test_eip155_avatar() {
        // TODO: implement
        assert_eq!(0, 0);
    }
}
