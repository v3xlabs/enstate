use ethers_core::{
    abi::{ParamType, Token},
    types::H256,
};
use ethers_core::types::U256;
use hex_literal::hex;
use lazy_static::lazy_static;
use thiserror::Error;
use tracing::info;

use crate::models::eip155::{EIP155ContractType, resolve_eip155};
use crate::models::multicoin::cointype::evm::ChainId;

use super::{abi_decode_universal_ccip, ENSLookupError, LookupState};

lazy_static! {
    static ref IPFS_REGEX: regex::Regex =
        regex::Regex::new(r"^ipfs://(ip[fn]s/)?([0-9a-zA-Z]+(/.*)?)")
            .expect("should be a valid regex");
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

pub fn function_selector() -> [u8; 4] {
    hex!("59d1d43c")
}

pub fn calldata(namehash: &H256, record: &str) -> Vec<u8> {
    let data = ethers_core::abi::encode(&[
        Token::FixedBytes(namehash.as_fixed_bytes().to_vec()),
        Token::String(record.to_string()),
    ]);

    [&function_selector() as &[u8], &data].concat()
}

pub async fn decode(data: &[u8], state: &LookupState) -> Result<String, ENSLookupError> {
    let decoded_abi = abi_decode_universal_ccip(data, &[ParamType::String])?;

    let Some(Token::String(value)) = decoded_abi.first() else {
        return Err(ENSLookupError::AbiDecodeError);
    };

    if let Some(captures) = IPFS_REGEX.captures(value) {
        let hash = captures.get(2).unwrap().as_str();

        return Ok(format!("{gateway}{hash}", gateway = state.ipfs_gateway));
    }

    let Some(captures) = EIP155_REGEX.captures(value) else {
        return Ok(value.to_string());
    };

    let (Some(chain_id), Some(contract_type), Some(contract_address), Some(token_id)) = (
        captures.get(1),
        captures.get(2),
        captures.get(3),
        captures.get(4),
    ) else {
        return Err(ENSLookupError::AbiDecodeError);
    };

    let chain_id = chain_id
        .as_str()
        .parse::<u64>()
        .map_err(|err| ImageLookupError::FormatError(err.to_string()))?;

    let token_id = U256::from_dec_str(token_id.as_str())
        .map_err(|err| ImageLookupError::FormatError(err.to_string()))?;

    let contract_type = match contract_type.as_str() {
        "erc721" => EIP155ContractType::ERC721,
        "erc1155" => EIP155ContractType::ERC1155,
        _ => return Err(ImageLookupError::FormatError("invalid contract type".to_string()).into()),
    };

    let contract_address = contract_address.as_str();

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
        state,
    )
    .await?;

    Ok(resolved_uri)
}

#[cfg(test)]
mod tests {
    use ethers::providers::namehash;

    use crate::models::lookup::ENSLookup;

    #[test]
    fn test_calldata_avatar() {
        assert_eq!(
            ENSLookup::StaticImage("avatar").calldata(&namehash("luc.eth")),
            hex_literal::hex!("59d1d43ce1e7bcf2ca33c28a806ee265cfedf02fedf1b124ca73b2203ca80cc7c91a02ad000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000066176617461720000000000000000000000000000000000000000000000000000")
        );
    }

    #[test]
    fn test_eip155_avatar() {
        // TODO: implement
        assert_eq!(0, 0);
    }
}
