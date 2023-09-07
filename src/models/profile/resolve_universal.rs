use crate::models::profile::Profile;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{transaction::eip2718::TypedTransaction, Address, Bytes},
};
use ethers_ccip_read::utils::dns_encode;
use ethers_contract::abigen;
use ethers_core::abi::{ParamType, Token};

use super::error::ProfileError;

abigen!(
    IUResolver,
    r#"[
        function resolve(bytes name, bytes[] data) external view returns (bytes[], address)
    ]"#,
);

impl Profile {
    pub async fn resolve_universal(
        name: &str,
        data: Vec<Vec<u8>>,
        provider: Provider<Http>,
    ) -> Result<(Vec<Vec<u8>>, Address), ProfileError> {
        // Setup address of universal resolver
        // TODO: Figure out way to only have to do this once
        let universal_address = "0xc0497E381f536Be9ce14B0dD3817cBcAe57d2F62"
            .parse::<Address>()
            .unwrap();

        // Prepare the variables
        let dns_encoded_node = dns_encode(name).unwrap();
        let wildcard_data = data.into_iter().map(Token::Bytes).collect();

        let encoded_data = ethers_core::abi::encode(&[
            Token::Bytes(dns_encoded_node),
            Token::Array(wildcard_data),
        ]);

        // resolve(bytes node, bytes[] data)
        let resolve_selector = "206c74c9";

        // Create the transaction
        let mut typed_transaction = TypedTransaction::default();

        // Prepare transaction data
        let transaction_data = [hex::decode(resolve_selector).unwrap(), encoded_data].concat();

        // Setup the transaction
        typed_transaction.set_to(universal_address);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::providers::namehash;

    #[tokio::test]
    async fn test_resolve_universal() {
        let namehash = namehash("luc.eth");
        let data = super::super::Profile::calldata_address(&namehash);
        let data2 = super::super::Profile::calldata_text(&namehash, "avatar");
        let data3 = super::super::Profile::calldata_text(&namehash, "com.github");
        let data4 = super::super::Profile::calldata_text(&namehash, "com.discord");
        let data5 = super::super::Profile::calldata_text(&namehash, "com.twitter");
        let data6 = super::super::Profile::calldata_text(&namehash, "timezone");

        let provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth").unwrap();

        let res = Profile::resolve_universal(
            "luc.eth",
            vec![data, data2, data3, data4, data5, data6],
            provider,
        )
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
