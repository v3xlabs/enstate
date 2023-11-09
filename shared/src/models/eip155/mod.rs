use std::sync::Arc;

use ethers::providers::{Http, Provider};
use ethers_core::{
    abi::{Token, ParamType},
    types::{transaction::eip2718::TypedTransaction, Bytes, H160, U256},
};
use thiserror::Error;
use tracing::info;

use super::{lookup::ENSLookupError, ipfs::IPFSURLUnparsed};

#[derive(Error, Debug)]
pub enum EIP155Error {
    #[error("Unsupported chain: {0}")]
    UnparsableChain(String),
    #[error("Unsupported chain: {0}")]
    UnsupportedChain(u64),
}

impl From<EIP155Error> for ENSLookupError {
    fn from(value: EIP155Error) -> Self {
        match value {
            EIP155Error::UnsupportedChain(chain_id) => {
                ENSLookupError::Unsupported(format!("Chain ID: {}", chain_id))
            }
            EIP155Error::UnparsableChain(chain_id) => {
                ENSLookupError::Unsupported(format!("Chain ID: {}", chain_id))
            }
        }
    }
}

pub async fn resolve_eip155(
    chain_id: &str,
    contract_type: &str,
    contract_address: &str,
    token_id: &str,
    provider: Arc<Provider<Http>>,
) -> Result<String, EIP155Error> {
    let chain_id: u64 = chain_id
        .parse()
        .map_err(|_| EIP155Error::UnparsableChain(chain_id.to_string()))?;

    // Check if chain_id is supported
    if chain_id != 1 {
        return Err(EIP155Error::UnsupportedChain(chain_id));
    }

    let token_id = U256::from_dec_str(token_id).unwrap();

    let mut typed_transaction = TypedTransaction::default();

    let encoded_data =
        ethers_core::abi::encode(&[Token::Int(token_id)]);

    let resolve_selector = match contract_type {
        "erc721" => {
            hex_literal::hex!("c87b56dd").to_vec()
        }
        "erc1155" => {
            hex_literal::hex!("0e89341c").to_vec()
        }
        _ => {
            return Err(EIP155Error::UnsupportedChain(chain_id));
        }
    };

    // Prepare transaction data
    let transaction_data: Vec<u8> = [resolve_selector, encoded_data].concat();

    typed_transaction.set_to(contract_address.parse::<H160>().unwrap());
    typed_transaction.set_data(Bytes::from(transaction_data));

    let res = provider.call_raw(&typed_transaction).await.unwrap();

    let res_data = res.to_vec();

    let result = ethers_core::abi::decode(
        &[
            ParamType::String,
        ], res_data.as_slice()).unwrap();

    // Token metadata url (Unvalidated)
    // Example url: https://my.nft.metadata.test/1234/2257
    // Content: json encoded {name: "", description: "", image: "", ...}
    let token_metadata_url = result.get(0).unwrap().to_string();

    // TODO: Validate URL here
    let token_metadata_url = IPFSURLUnparsed::from_unparsed(token_metadata_url);

    let token_metadata = token_metadata_url.fetch().await.unwrap();

    Ok(token_metadata.image)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // async fn test_calldata_avatar() {
    //     let provider = Arc::new(Provider::<Http>::try_from("https://rpc.ankr.com/eth").unwrap());
    //     let data = resolve_eip155(
    //         "1",
    //         "erc1155",
    //         "0x495f947276749ce646f68ac8c248420045cb7b5e",
    //         "8112316025873927737505937898915153732580103913704334048512380490797008551937",
    //         provider,
    //     )
    //     .await
    //     .unwrap();

    //     assert_eq!(data, "".to_string());
    // }

    #[tokio::test]
    async fn test_calldata_avatar() {
        let provider = Arc::new(Provider::<Http>::try_from("https://rpc.ankr.com/eth").unwrap());
        let data = resolve_eip155(
            "1",
            "erc721",
            "0xc92ceddfb8dd984a89fb494c376f9a48b999aafc",
            "2257",
            provider,
        )
        .await
        .unwrap();

        assert_eq!(data, "https://creature.mypinata.cloud/ipfs/QmeZGc1CL3eb9QJatKXTGT7ekgLMq9FyZUWckQ4oWdc53a/2257.jpg".to_string());
    }


}
