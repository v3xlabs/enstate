use ethers::prelude::ProviderError::JsonRpcClientError;
use ethers::{
    providers::{namehash, Http, Middleware, Provider},
    types::{transaction::eip2718::TypedTransaction, Address, Bytes},
};
use ethers_ccip_read::{CCIPReadMiddleware, CCIPReadMiddlewareError};
use ethers_contract::abigen;
use ethers_core::abi::{ParamType, Token};
use lazy_static::lazy_static;

use crate::models::lookup::ENSLookup;
use crate::utils::dns::dns_encode;

use super::profile::error::ProfileError;

abigen!(
    IUResolver,
    r#"[
        function resolve(bytes name, bytes[] data) external view returns (bytes[], address)
    ]"#,
);

lazy_static! {
    // Setup address of universal resolver
    static ref UNIVERSAL_RESOLVER_ADDRESS: Address = "0xc0497E381f536Be9ce14B0dD3817cBcAe57d2F62"
        .parse::<Address>()
        .expect("UNIVERSAL_RESOLVER_ADDRESS should be a valid address");
}

const MAGIC_UNIVERSAL_RESOLVER_ERROR_MESSAGE: &str =
    "execution reverted: UniversalResolver: Wildcard on non-extended resolvers is not supported";

pub async fn resolve_universal(
    name: &str,
    data: &[Box<dyn ENSLookup + Send + Sync>],
    provider: &CCIPReadMiddleware<Provider<Http>>,
) -> Result<(Vec<Vec<u8>>, Address), ProfileError> {
    let name_hash = namehash(name);

    // Prepare the variables
    let dns_encoded_node = dns_encode(name).map_err(ProfileError::DNSEncodeError)?;

    let wildcard_data = data
        .iter()
        .map(|x| x.calldata(&name_hash))
        .map(Token::Bytes)
        .collect();

    let encoded_data =
        ethers_core::abi::encode(&[Token::Bytes(dns_encoded_node), Token::Array(wildcard_data)]);

    // resolve(bytes node, bytes[] data)
    let resolve_selector = hex_literal::hex!("206c74c9").to_vec();

    // Create the transaction
    let mut typed_transaction = TypedTransaction::default();

    // Prepare transaction data
    let transaction_data = [resolve_selector, encoded_data].concat();

    // Setup the transaction
    typed_transaction.set_to(*UNIVERSAL_RESOLVER_ADDRESS);
    typed_transaction.set_data(Bytes::from(transaction_data));

    // Call the transaction
    let res = provider
        .call(&typed_transaction, None)
        .await
        .map_err(|err| {
            let CCIPReadMiddlewareError::MiddlewareError(provider_error) = err else {
                return ProfileError::CCIPError(err);
            };

            let JsonRpcClientError(rpc_err) = &provider_error else {
                return ProfileError::RPCError(provider_error);
            };

            if matches!(rpc_err.as_error_response(), Some(rpc_err_raw) if rpc_err_raw.message == MAGIC_UNIVERSAL_RESOLVER_ERROR_MESSAGE) {
                return ProfileError::NotFound;
            }

            ProfileError::RPCError(provider_error)
        })?;

    // Abi Decode
    let result = ethers_core::abi::decode(
        &[
            ParamType::Array(Box::new(ParamType::Bytes)),
            ParamType::Address,
        ],
        res.as_ref(),
    )
    .map_err(|_| ProfileError::ImplementationError("ABI decode failed".to_string()))?;

    if result.len() < 2 {
        // should never trigger
        return Err(ProfileError::ImplementationError("".to_string()));
    }

    let result_data = result.get(0).expect("result[0] should exist").clone();
    let result_address = result.get(1).expect("result[1] should exist").clone();

    let resolver = result_address
        .into_address()
        .expect("result[1] should be an address");

    if resolver.is_zero() {
        return Err(ProfileError::NotFound);
    }

    Ok((
        result_data
            .into_array()
            .expect("result[0] should be an array")
            .into_iter()
            .map(|t| t.into_bytes().expect("result[0] elements should be bytes"))
            .collect(),
        resolver,
    ))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ethers::providers::{Http, Provider};
    use ethers_ccip_read::CCIPReadMiddleware;
    use ethers_core::abi::ParamType;
    use ethers_core::types::Address;

    use crate::models::lookup::addr::Addr;
    use crate::models::lookup::text::Text;
    use crate::models::lookup::ENSLookup;
    use crate::models::universal_resolver;

    #[tokio::test]
    async fn test_resolve_universal() {
        let provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth").unwrap();

        let calldata: Vec<Box<dyn ENSLookup + Send + Sync>> = vec![
            Addr {}.to_boxed(),
            Text::from("com.discord").to_boxed(),
            Text::from("com.github").to_boxed(),
            Text::from("com.twitter").to_boxed(),
            Text::from("org.telegram").to_boxed(),
            Text::from("location").to_boxed(),
        ];

        let res = universal_resolver::resolve_universal(
            "antony.sh",
            &calldata,
            &CCIPReadMiddleware::new(provider),
        )
        .await
        .unwrap();

        let address = ethers_core::abi::decode(&[ParamType::Address], &res.0[0])
            .unwrap()
            .get(0)
            .unwrap()
            .clone()
            .into_address()
            .unwrap();

        let text_response: Vec<String> = res.0[1..]
            .iter()
            .map(|t| {
                ethers_core::abi::decode(&[ParamType::String], t)
                    .unwrap()
                    .get(0)
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
