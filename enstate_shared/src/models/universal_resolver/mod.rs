use ethers::{
    providers::{Http, Middleware, namehash, Provider},
    types::{Address, Bytes, transaction::eip2718::TypedTransaction},
};
use ethers_ccip_read::utils::dns_encode;
use ethers_contract::abigen;
use ethers_core::abi::{ParamType, Token};
use lazy_static::lazy_static;

use crate::models::lookup::ENSLookup;

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
    name: String,
    data: &Vec<Box<dyn ENSLookup + Send + Sync>>,
    provider: Provider<Http>,
) -> Result<(Vec<Vec<u8>>, Address), ProfileError> {
    let name_hash = namehash(name.as_str());

    // Prepare the variables
    let dns_encoded_node = dns_encode(name.as_str()).unwrap();
    let wildcard_data = data
        .into_iter()
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
    let res = provider.call(&typed_transaction, None).await.unwrap();

    let res_data = res.to_vec();

    // Abi Decode
    let result = ethers_core::abi::decode(
        &[
            ParamType::Array(Box::new(ParamType::Bytes)),
            ParamType::Address,
        ],
        res_data.as_slice(),
    )
        .unwrap();

    let result_datas = result.get(0).unwrap().clone();
    let result_address = result.get(1).unwrap().clone();

    Ok((
        result_datas
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
    #[tokio::test]
    async fn test_resolve_universal() {
        // let namehash = namehash("luc.eth");
        // let data = super::super::Profile::calldata_address(&namehash);
        // let data2 = super::super::Profile::calldata_text(&namehash, "avatar");
        // let data3 = super::super::Profile::calldata_text(&namehash, "com.github");
        // let data4 = super::super::Profile::calldata_text(&namehash, "com.discord");
        // let data5 = super::super::Profile::calldata_text(&namehash, "com.twitter");
        // let data6 = super::super::Profile::calldata_text(&namehash, "timezone");

        // let provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth").unwrap();

        // let res = Profile::resolve_universal(
        //     "luc.eth",
        //     vec![],
        //     provider,
        // )
        // .await
        // .unwrap();

        // println!("{:?}", res);

        // let text_response: Vec<String> = res.0[1..]
        //     .iter()
        //     .map(|t| {
        //         ethers_core::abi::decode(&[ParamType::String], t)
        //             .unwrap()
        //             .get(0)
        //             .unwrap()
        //             .clone()
        //             .into_string()
        //             .unwrap()
        //     })
        //     .collect();

        // println!("{:?}", text_response);

        // // assert_eq!(res, Err(ProfileError::NotFound));
    }
}
