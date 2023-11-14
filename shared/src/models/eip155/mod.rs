use ethers::providers::{Http, Provider, ProviderError};
use ethers_core::{
    abi::{ParamType, Token},
    types::{transaction::eip2718::TypedTransaction, Bytes, H160, U256},
};
use thiserror::Error;
use tracing::info;

use crate::models::ipfs::{URLFetchError, OPENSEA_BASE_PREFIX};

use super::ipfs::IPFSURLUnparsed;

#[derive(Error, Debug)]
pub enum EIP155Error {
    #[error("Unsupported chain: {0}")]
    UnsupportedChain(u64),

    // when either chain_id or token_id is to big to be parsed as u64/U256 respectively
    #[error("Format error")]
    FormatError,

    #[error("RPC error: {0}")]
    RPCError(#[from] ProviderError),

    #[error("Metadata fetch error: {0}")]
    MetadataFetchError(#[from] URLFetchError),

    #[error("Implementation error: {0}")]
    ImplementationError(String),

    #[error("Other error")]
    Other,
}

pub async fn resolve_eip155(
    chain_id: &str,
    contract_type: &str,
    contract_address: &str,
    token_id: &str,
    provider: &Provider<Http>,
    opensea_api_key: &str,
) -> Result<String, EIP155Error> {
    let chain_id: u64 = chain_id.parse().map_err(|_| EIP155Error::FormatError)?;

    // Check if chain_id is supported
    // TODO: multiple chains
    if chain_id != 1 {
        return Err(EIP155Error::UnsupportedChain(chain_id));
    }

    let token_id = U256::from_dec_str(token_id).map_err(|_| EIP155Error::FormatError)?;

    let mut typed_transaction = TypedTransaction::default();

    let encoded_data = ethers_core::abi::encode(&[Token::Int(token_id)]);

    let resolve_selector = match contract_type {
        "erc721" => hex_literal::hex!("c87b56dd").to_vec(),
        "erc1155" => hex_literal::hex!("0e89341c").to_vec(),
        _ => {
            return Err(EIP155Error::UnsupportedChain(chain_id));
        }
    };

    // Prepare transaction data
    let transaction_data: Vec<u8> = [resolve_selector, encoded_data].concat();

    let contract_h160 = contract_address.parse::<H160>().map_err(|_| {
        EIP155Error::ImplementationError("contract_address not a correct H160".to_string())
    })?;

    typed_transaction.set_to(contract_h160);
    typed_transaction.set_data(Bytes::from(transaction_data));

    let res = provider.call_raw(&typed_transaction).await?;

    let res_data = res.to_vec();

    let result = ethers_core::abi::decode(&[ParamType::String], res_data.as_slice())
        .map_err(|_| EIP155Error::ImplementationError("ABI decode failed".to_string()))?;

    // Token metadata url (Unvalidated)
    // Example url: https://my.nft.metadata.test/1234/2257
    // Content: json encoded {name: "", description: "", image: "", ...}
    let mut token_metadata_url = result
        .get(0)
        // should never trigger
        .ok_or_else(|| EIP155Error::ImplementationError("".to_string()))?
        .to_string();

    // Replace 0x{id} with token_id (opensea specific)
    if token_metadata_url.starts_with(OPENSEA_BASE_PREFIX) {
        token_metadata_url = token_metadata_url.replace("0x{id}", &token_id.to_string());
    }

    // Replace {id} with token_id
    token_metadata_url = token_metadata_url.replace("{id}", &token_id.to_string());

    info!("Token Metadata URL: {}", token_metadata_url);

    // TODO: Validate URL here
    let token_metadata_url = IPFSURLUnparsed::from_unparsed(token_metadata_url);

    let token_metadata = token_metadata_url.fetch(opensea_api_key).await?;

    let image = token_metadata.image.ok_or(EIP155Error::Other)?;

    info!("Image: {}", image);

    let token_image_url = IPFSURLUnparsed::from_unparsed(image).to_url_or_gateway();

    Ok(token_image_url)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[tokio::test]
    async fn test_calldata_avatar() {
        let provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth").unwrap();
        let opensea_api_key = env::var("OPENSEA_API_KEY").unwrap().to_string();

        let data = resolve_eip155(
            "1",
            "erc721",
            "0xc92ceddfb8dd984a89fb494c376f9a48b999aafc",
            "2257",
            &provider,
            &opensea_api_key,
        )
        .await
        .unwrap();

        assert_eq!(data, "https://creature.mypinata.cloud/ipfs/QmeZGc1CL3eb9QJatKXTGT7ekgLMq9FyZUWckQ4oWdc53a/2257.jpg".to_string());
    }
}
