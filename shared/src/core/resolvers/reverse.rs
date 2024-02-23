use ethers::middleware::Middleware;
use ethers::prelude::{Address, H256, ProviderError};
use ethers::providers::{namehash, Provider};
use ethers_contract::providers::Http;
use ethers_core::abi;
use ethers_core::abi::{AbiEncode, ParamType, Token};
use ethers_core::types::Bytes;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use hex_literal::hex;
use lazy_static::lazy_static;
use thiserror::Error;

use crate::core::CCIPProvider;
use crate::core::resolvers::universal::resolve_universal;
use crate::models::lookup::{addr, ENSLookup};

#[derive(Error, Debug)]
pub enum ReverseResolveError {
    #[error("Address doesn't have a primary name")]
    MissingPrimaryName,

    #[error("Address on name doesn't match with lookup address")]
    AddressMismatch,

    #[error("Failed to lookup address for reverse record: {0}")]
    AddressLookupError(String),

    #[error("RPC provider error: {0}")]
    RPCError(#[from] ProviderError),

    #[error("ABI decode error: {0}")]
    AbiDecodeError(#[from] abi::Error),
}

lazy_static! {
    static ref BASE_REGISTRY: Address = "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e"
        .parse()
        .expect("should be a valid address");
}

const REVERSE_NAME_SUFFIX: &str = "addr.reverse";

const RESOLVE_SELECTOR: [u8; 4] = hex!("0178b8bf");
const NAME_SELECTOR: [u8; 4] = hex!("691f3431");

async fn find_resolver(
    rpc: &Provider<Http>,
    namehash: &H256,
) -> Result<Address, ReverseResolveError> {
    let mut transaction = TypedTransaction::default();

    transaction.set_to(*BASE_REGISTRY);

    let encoded = abi::encode(&[Token::FixedBytes(namehash.encode())]);
    transaction.set_data(Bytes::from(
        [&RESOLVE_SELECTOR, encoded.as_slice()].concat(),
    ));

    let res = rpc.call(&transaction, None).await?;

    let address = abi::decode(&[ParamType::Address], &res)?
        .first()
        .unwrap()
        .clone()
        .into_address()
        .unwrap();

    Ok(address)
}

pub async fn resolve_reverse(
    rpc: &CCIPProvider,
    address: &Address,
    universal_resolver: &Address,
) -> Result<String, ReverseResolveError> {
    let reverse_namehash = namehash(&format!(
        "{}.{REVERSE_NAME_SUFFIX}",
        hex::encode(address.as_bytes())
    ));

    let resolver = find_resolver(rpc.inner(), &reverse_namehash).await?;

    if resolver.is_zero() {
        return Err(ReverseResolveError::MissingPrimaryName);
    }

    let mut transaction = TypedTransaction::default();

    transaction.set_to(resolver);

    let encoded = abi::encode(&[Token::FixedBytes(reverse_namehash.encode())]);
    transaction.set_data(Bytes::from([&NAME_SELECTOR, encoded.as_slice()].concat()));

    let res = rpc.inner().call(&transaction, None).await?;

    let name = abi::decode(&[ParamType::String], &res)?
        .first()
        .unwrap()
        .clone()
        .into_string()
        .unwrap();

    let (mut res, _, _) = resolve_universal(&name, &[ENSLookup::Addr], rpc, universal_resolver)
        .await
        .map_err(|err| ReverseResolveError::AddressLookupError(err.to_string()))?;

    let addr_result = res.remove(0);
    if !addr_result.success {
        return Err(ReverseResolveError::AddressMismatch);
    }

    let decoded = addr::decode(&addr_result.data)
        .await
        .map_err(|err| ReverseResolveError::AddressLookupError(err.to_string()))?;

    if !matches!(decoded.parse::<Address>(), Ok(ref parsed) if parsed == address) {
        return Err(ReverseResolveError::AddressMismatch);
    }

    Ok(name)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ethers::prelude::{Http, Provider};
    use ethers_ccip_read::CCIPReadMiddleware;

    use crate::core::resolvers::reverse::resolve_reverse;

    #[tokio::test]
    async fn test() {
        let provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth").unwrap();

        let provider = CCIPReadMiddleware::new(Arc::new(provider));

        assert_eq!(
            resolve_reverse(
                &provider,
                &"0xb8c2C29ee19D8307cb7255e1Cd9CbDE883A267d5"
                    .parse()
                    .unwrap(),
                &"0x8cab227b1162f03b8338331adaad7aadc83b895e"
                    .parse()
                    .unwrap(),
            )
                .await
                .ok(),
            Some("nick.eth".to_string())
        );

        assert_eq!(
            resolve_reverse(
                &provider,
                &"0x2B5c7025998f88550Ef2fEce8bf87935f542C190"
                    .parse()
                    .unwrap(),
                &"0x8cab227b1162f03b8338331adaad7aadc83b895e"
                    .parse()
                    .unwrap(),
            )
                .await
                .ok(),
            Some("antony.sh".to_string())
        );
    }
}
