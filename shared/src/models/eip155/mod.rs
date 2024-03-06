use ethers::middleware::Middleware;
use ethers::providers::ProviderError;
use ethers_core::{
    abi::{ParamType, Token},
    types::{Bytes, H160, transaction::eip2718::TypedTransaction, U256},
};
use thiserror::Error;
use tracing::info;
use tracing::instrument;

use crate::models::eip155::url::{OPENSEA_BASE_PREFIX, URLFetchError, URLParseError, URLUnparsed};
use crate::models::lookup::LookupState;
use crate::models::multicoin::cointype::evm::ChainId;

mod url;

#[derive(Error, Debug)]
pub enum EIP155Error {
    #[error("Unsupported chain: {0}")]
    UnsupportedChain(u64),

    #[error("RPC error: {0}")]
    RPCError(#[from] ProviderError),

    #[error("Metadata fetch error: {0}")]
    MetadataFetchError(#[from] URLFetchError),

    #[error("Not a valid URL: {0}")]
    InvalidURL(#[from] URLParseError),

    #[error("Implementation error: {0}")]
    ImplementationError(String),

    #[error("Other error")]
    Other,
}

pub enum EIP155ContractType {
    ERC721,
    ERC1155,
}

impl EIP155ContractType {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for EIP155ContractType {
    fn as_ref(&self) -> &str {
        match self {
            Self::ERC721 => "erc721",
            Self::ERC1155 => "erc1155",
        }
    }
}

#[instrument(skip_all)]
pub async fn resolve_eip155(
    chain_id: ChainId,
    contract_type: EIP155ContractType,
    contract_address: &str,
    token_id: U256,
    state: &LookupState,
) -> Result<String, EIP155Error> {
    let chain_id: u64 = chain_id.into();

    // Check if chain_id is supported
    // TODO: multiple chains
    if chain_id != 1 {
        return Err(EIP155Error::UnsupportedChain(chain_id));
    }

    let mut typed_transaction = TypedTransaction::default();

    let encoded_data = ethers_core::abi::encode(&[Token::Int(token_id)]);

    let resolve_selector = match contract_type {
        EIP155ContractType::ERC721 => hex_literal::hex!("c87b56dd").to_vec(),
        EIP155ContractType::ERC1155 => hex_literal::hex!("0e89341c").to_vec(),
    };

    // Prepare transaction data
    let transaction_data: Vec<u8> = [resolve_selector, encoded_data].concat();

    let contract_h160 = contract_address.parse::<H160>().map_err(|_| {
        EIP155Error::ImplementationError("contract_address not a correct H160".to_string())
    })?;

    typed_transaction.set_to(contract_h160);
    typed_transaction.set_data(Bytes::from(transaction_data));

    let res = state.rpc.provider().call_raw(&typed_transaction).await?;

    let res_data = res.to_vec();

    let result = ethers_core::abi::decode(&[ParamType::String], res_data.as_slice())
        .map_err(|_| EIP155Error::ImplementationError("ABI decode failed".to_string()))?;

    // Token metadata url (Unvalidated)
    // Example url: https://my.nft.metadata.test/1234/2257
    // Content: json encoded {name: "", description: "", image: "", ...}
    let mut token_metadata_url = result
        .first()
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
    let token_metadata_url = URLUnparsed::from_unparsed(&token_metadata_url)?;

    let token_metadata = token_metadata_url.fetch(state).await?;

    let image = token_metadata.image.ok_or(EIP155Error::Other)?;

    info!("Image: {}", image);

    let token_image_url = URLUnparsed::from_unparsed(&image)?.to_url_or_ipfs_gateway(state);

    Ok(token_image_url)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Arc;

    use ethers::middleware::MiddlewareBuilder;
    use ethers::providers::{Http, Provider};
    use ethers_ccip_read::CCIPReadMiddleware;

    use super::*;

    #[tokio::test]
    async fn test_calldata_avatar_erc721() {
        let provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth")
            .unwrap()
            .wrap_into(|it| CCIPReadMiddleware::new(Arc::from(it)));
        let opensea_api_key = env::var("OPENSEA_API_KEY").unwrap().to_string();

        let state = LookupState {
            rpc: Arc::new(provider),
            opensea_api_key,
            ipfs_gateway: "https://ipfs.io/ipfs/".to_string(),
            arweave_gateway: "https://arweave.net/".to_string(),
        };

        let data = resolve_eip155(
            ChainId::Ethereum,
            EIP155ContractType::ERC721,
            "0xc92ceddfb8dd984a89fb494c376f9a48b999aafc",
            U256::from_dec_str("2257").unwrap(),
            &state,
        )
        .await
        .unwrap();

        assert_eq!(data, "https://creature.mypinata.cloud/ipfs/QmeZGc1CL3eb9QJatKXTGT7ekgLMq9FyZUWckQ4oWdc53a/2257.jpg".to_string());
    }

    #[tokio::test]
    async fn test_calldata_avatar_erc1155() {
        let provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth")
            .unwrap()
            .wrap_into(|it| CCIPReadMiddleware::new(Arc::from(it)));
        let opensea_api_key = env::var("OPENSEA_API_KEY").unwrap().to_string();

        let state = LookupState {
            rpc: Arc::new(provider),
            opensea_api_key,
            ipfs_gateway: "https://ipfs.io/ipfs/".to_string(),
            arweave_gateway: "https://arweave.net/".to_string(),
        };

        let data = resolve_eip155(
            ChainId::Ethereum,
            EIP155ContractType::ERC1155,
            "0xb32979486938aa9694bfc898f35dbed459f44424",
            U256::from_dec_str("10063").unwrap(),
            &state,
        )
        .await
        .unwrap();

        assert_eq!(
            data,
            "https://ipfs.io/ipfs/QmSP4nq9fnN9dAiCj42ug9Wa79rqmQerZXZch82VqpiH7U/image.gif"
                .to_string()
        );
    }

    #[tokio::test]
    async fn test_calldata_avatar_erc1155_opensea() {
        let provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth")
            .unwrap()
            .wrap_into(|it| CCIPReadMiddleware::new(Arc::from(it)));
        let opensea_api_key = env::var("OPENSEA_API_KEY").unwrap().to_string();

        let state = LookupState {
            rpc: Arc::new(provider),
            opensea_api_key,
            ipfs_gateway: "https://ipfs.io/ipfs/".to_string(),
            arweave_gateway: "https://arweave.net/".to_string(),
        };

        let data = resolve_eip155(
            ChainId::Ethereum,
            EIP155ContractType::ERC1155,
            "0x495f947276749ce646f68ac8c248420045cb7b5e",
            U256::from_dec_str(
                "8112316025873927737505937898915153732580103913704334048512380490797008551937",
            )
            .unwrap(),
            &state,
        )
        .await
        .unwrap();

        assert_eq!(data, "https://i.seadn.io/gae/hKHZTZSTmcznonu8I6xcVZio1IF76fq0XmcxnvUykC-FGuVJ75UPdLDlKJsfgVXH9wOSmkyHw0C39VAYtsGyxT7WNybjQ6s3fM3macE?w=500&auto=format".to_string());
    }
}
