use std::vec;

use ethers::prelude::ProviderError::JsonRpcClientError;
use ethers::{
    providers::namehash,
    types::{transaction::eip2718::TypedTransaction, Address, Bytes},
};
use ethers_ccip_read::{CCIPReadMiddlewareError, CCIPRequest};
use ethers_core::abi;
use ethers_core::abi::{ParamType, Token};
use ethers_core::types::H160;
use hex_literal::hex;
use lazy_static::lazy_static;

use crate::core::error::ProfileError;
use crate::core::CCIPProvider;
use crate::models::lookup::{addr, ENSLookup};
use crate::utils::dns::dns_encode;
use crate::utils::vec::dedup_ord;

lazy_static! {
    static ref OFFCHAIN_DNS_RESOLVER: Address =
        Address::from(hex!("F142B308cF687d4358410a4cB885513b30A42025"));
}

pub struct UniversalResolverResult {
    pub(crate) success: bool,
    pub(crate) data: Vec<u8>,
}

pub async fn resolve_universal(
    name: &str,
    data: &[ENSLookup],
    provider: &CCIPProvider,
    universal_resolver: &H160,
) -> Result<(Vec<UniversalResolverResult>, Address, Vec<String>), ProfileError> {
    let name_hash = namehash(name);

    // Prepare the variables
    let dns_encoded_node = dns_encode(name).map_err(ProfileError::DNSEncodeError)?;

    // FIXME: don't force address lookup
    let data = [&[ENSLookup::Addr] as &[ENSLookup], data].concat();

    let wildcard_data = data
        .iter()
        .map(|it| it.calldata(&name_hash))
        .map(Token::Bytes)
        .collect();

    let encoded_data = abi::encode(&[Token::Bytes(dns_encoded_node), Token::Array(wildcard_data)]);

    // resolve(bytes node, bytes[] data)
    let resolve_selector = hex_literal::hex!("206c74c9").to_vec();

    // Create the transaction
    let mut typed_transaction = TypedTransaction::default();

    // Prepare transaction data
    let transaction_data = [resolve_selector, encoded_data].concat();

    // Set up the transaction
    typed_transaction.set_to(*universal_resolver);
    typed_transaction.set_data(Bytes::from(transaction_data));

    // Call the transaction
    let (res, ccip_requests) =
        provider
            .call_ccip(&typed_transaction, None)
            .await
            .map_err(|err| {
                let CCIPReadMiddlewareError::MiddlewareError(provider_error) = err else {
                    return ProfileError::CCIPError(err);
                };

                let JsonRpcClientError(rpc_err) = &provider_error else {
                    return ProfileError::RPCError(provider_error);
                };

                // TODO: better error handling
                if rpc_err.as_error_response().is_some() {
                    return ProfileError::NotFound;
                }

                ProfileError::RPCError(provider_error)
            })?;

    // Abi Decode
    let result = abi::decode(
        &[
            ParamType::Array(Box::new(ParamType::Tuple(vec![
                ParamType::Bool,
                ParamType::Bytes,
            ]))),
            ParamType::Address,
        ],
        res.as_ref(),
    )
    .map_err(|_| ProfileError::ImplementationError("ABI decode failed".to_string()))?;

    if result.len() < 2 {
        // should never trigger
        return Err(ProfileError::ImplementationError("".to_string()));
    }

    let result_data = result.first().expect("result[0] should exist").clone();
    let result_address = result.get(1).expect("result[1] should exist").clone();

    let resolver = result_address
        .into_address()
        .expect("result[1] should be an address");

    let mut parsed: Vec<UniversalResolverResult> = result_data
        .into_array()
        .expect("result[0] should be an array")
        .into_iter()
        .map(|t| {
            let tuple = t.into_tuple().expect("result[0] should be a tuple");

            let success = tuple
                .first()
                .expect("result[0][0] should exist")
                .clone()
                .into_bool()
                .expect("result[0][0] should be a boolean");
            let data = tuple
                .get(1)
                .expect("result[0][1] should exist")
                .clone()
                .into_bytes()
                .expect("result[0][1] elements should be bytes");

            UniversalResolverResult { success, data }
        })
        .collect();

    let addr = addr::decode(&parsed.remove(0).data).await;

    // if we got a CCIP response, where the resolver is an OffchainDNSResolver and the address if zero
    //  it's a non-existing name
    if resolver.is_zero()
        || (resolver == *OFFCHAIN_DNS_RESOLVER
            && matches!(addr, Ok(ref addr) if addr == "0x0000000000000000000000000000000000000000"))
    {
        return Err(ProfileError::NotFound);
    }

    Ok((
        parsed,
        resolver,
        dedup_ord(
            &ccip_requests
                .iter()
                .flat_map(urls_from_request)
                .collect::<Vec<_>>(),
        ),
    ))
}

fn urls_from_request(request: &CCIPRequest) -> Vec<String> {
    if request.calldata.len() < 4 {
        return Vec::new();
    }

    let decoded = abi::decode(
        &[ParamType::Array(Box::new(ParamType::Tuple(vec![
            ParamType::Address,
            ParamType::Array(Box::new(ParamType::String)),
            ParamType::Bytes,
        ])))],
        &request.calldata[4..],
    )
    .unwrap_or_default();

    let Some(Token::Array(requests)) = decoded.first() else {
        return Vec::new();
    };

    requests
        .iter()
        .flat_map(|request| {
            let Token::Tuple(request) = request else {
                return Vec::new();
            };

            let Some(Token::Array(urls)) = request.get(1) else {
                return Vec::new();
            };

            urls.iter()
                .filter_map(|url| match url {
                    Token::String(url) => Some(url.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::sync::Arc;

    use ethers::providers::{Http, Provider};
    use ethers_ccip_read::CCIPReadMiddleware;
    use ethers_core::abi::ParamType;
    use ethers_core::types::Address;

    use crate::core::universal_resolver;
    use crate::models::lookup::ENSLookup;

    #[tokio::test]
    async fn test_resolve_universal() {
        let provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth").unwrap();

        let calldata: Vec<ENSLookup> = vec![
            ENSLookup::Addr,
            ENSLookup::StaticText("com.discord"),
            ENSLookup::StaticText("com.github"),
            ENSLookup::StaticText("com.twitter"),
            ENSLookup::StaticText("org.telegram"),
            ENSLookup::StaticText("location"),
        ];

        let res = universal_resolver::resolve_universal(
            "antony.sh",
            &calldata,
            &CCIPReadMiddleware::new(Arc::new(provider)),
            &Address::from_str("0x8cab227b1162f03b8338331adaad7aadc83b895e").unwrap(),
        )
        .await
        .unwrap();

        let address = ethers_core::abi::decode(&[ParamType::Address], &res.0[0].data)
            .unwrap()
            .first()
            .unwrap()
            .clone()
            .into_address()
            .unwrap();

        let text_response: Vec<String> = res.0[1..]
            .iter()
            .map(|t| {
                ethers_core::abi::decode(&[ParamType::String], &t.data)
                    .unwrap()
                    .first()
                    .unwrap()
                    .clone()
                    .into_string()
                    .unwrap()
            })
            .collect();

        // yes, I did make this test completely dependent on me 😈
        // TODO: make less dependent on a single person

        assert_eq!(
            address,
            Address::from_str("0x2B5c7025998f88550Ef2fEce8bf87935f542C190").unwrap()
        );
        assert_eq!(
            text_response,
            vec![
                "antony.sh",
                "Antony1060",
                "AntonyThe1060",
                "Antony1060",
                "Croatia"
            ]
        );
    }
}
