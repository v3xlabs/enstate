use ethers::{
    providers::{namehash, Http, Middleware, Provider},
    types::{transaction::eip2718::TypedTransaction, Address, Bytes},
};
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
    static ref UNIVERSAL_ADDRESS: Address = "0xc0497E381f536Be9ce14B0dD3817cBcAe57d2F62"
        .parse::<Address>()
        .unwrap();
}

pub async fn resolve_universal(
    name: &str,
    data: &[Box<dyn ENSLookup + Send + Sync>],
    provider: &Provider<Http>,
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
    typed_transaction.set_to(*UNIVERSAL_ADDRESS);
    typed_transaction.set_data(Bytes::from(transaction_data));

    // Call the transaction
    let res = provider.call(&typed_transaction, None).await?;

    let res_data = res.to_vec();

    // Abi Decode
    let result = ethers_core::abi::decode(
        &[
            ParamType::Array(Box::new(ParamType::Bytes)),
            ParamType::Address,
        ],
        res_data.as_slice(),
    )
    .map_err(|_| ProfileError::ImplementationError("ABI decode failed".to_string()))?;

    if result.len() < 2 {
        // should never trigger
        return Err(ProfileError::ImplementationError("".to_string()));
    }

    let result_data = result.get(0).unwrap().clone();
    let result_address = result.get(1).unwrap().clone();

    Ok((
        result_data
            .into_array()
            .unwrap()
            .into_iter()
            .map(|t| t.into_bytes().unwrap())
            .collect(),
        result_address.into_address().unwrap(),
    ))
}

#[cfg(test)]
mod tests {
    use ethers::providers::{Http, Provider};

    use crate::models::universal_resolver;

    async fn test_resolve_universal() {
        let provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth").unwrap();

        let res = universal_resolver::resolve_universal("luc.eth", &[], &provider)
            .await
            .unwrap();

        println!("{:?}", res);

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

        println!("{:?}", text_response);

        // assert_eq!(res, Err(ProfileError::NotFound));
    }
}
