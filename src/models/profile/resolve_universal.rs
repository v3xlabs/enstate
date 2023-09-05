use std::{ops::Add, sync::Arc};

use ethers::{
    providers::{namehash, Http, Middleware, Provider},
    types::{transaction::eip2718::TypedTransaction, Address, Bytes, H160, H256},
};
use ethers_ccip_read::{utils::dns_encode, CCIPReadMiddleware};
use ethers_contract::{abigen, Contract, ContractCall};
use ethers_core::{abi::{Token, ParamType}, types::Eip2930TransactionRequest, utils::hex::FromHex};
use tracing::info;

use crate::{abi::UResolver::UniversalResolver, models::profile::Profile};

use super::ProfileError;

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
    ) -> Result<Vec<String>, ProfileError> {
        let universal_address = "0xc0497E381f536Be9ce14B0dD3817cBcAe57d2F62"
            .parse::<Address>()
            .unwrap();

        println!("universal_address: {:?}", universal_address);
        println!("data: {:?}", data);

        let dns_encode_token = Token::Bytes(dns_encode(name).unwrap());
        let tx_data_token = Token::Array(data.into_iter().map(|d| Token::Bytes(d)).collect());

        let tokens = vec![dns_encode_token, tx_data_token];

        let encoded_data = ethers_core::abi::encode(&tokens);

        let resolve_selector = "206c74c9";

        let mut typed_transaction = TypedTransaction::default();

        typed_transaction.set_to(universal_address);
        typed_transaction.set_data(Bytes::from(
            [hex::decode(resolve_selector).unwrap(), encoded_data].concat(),
        ));

        let res = provider.call(&typed_transaction, None).await.unwrap();
        let res_data = res.to_vec();

        let result = ethers_core::abi::decode(&[ParamType::Array(Box::new(ParamType::Bytes)), ParamType::Address], res_data.as_slice()).unwrap();

        println!("result: {:?}", result);
        // let contract = IUResolver::new(universal_address, Arc::new(provider.inner()));

        // let node = namehash(name);
        // let node = Bytes::from(node.as_bytes().to_vec());

        // let abi: ethers_core::abi::Abi = serde_json::from_str(
        //     r#"[{"inputs":[{"internalType":"bytes","name":"name","type":"bytes"},{"internalType":"bytes[]","name":"data","type":"bytes[]"}],"name":"resolve","outputs":[{"internalType":"bytes[]","name":"","type":"bytes[]"},{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"}]"#,
        // )
        // .unwrap();

        // let contract = Contract::new(universal_address, abi, Arc::new(provider));

        // let res = contract
        //     .method::<(Bytes, Vec<Bytes>), (Vec<Bytes>, Address)>(
        //         "resolve",
        //         (
        //             node,
        //             vec![Bytes::from_hex(
        //                 "0x3b3b57dee1e7bcf2ca33c28a806ee265cfedf02fedf1b124ca73b2203ca80cc7c91a02ad",
        //             ).unwrap()],
        //         ),
        //     )
        //     .unwrap();

        // let res = res.call().await.unwrap();

        // let res = res.call().await.unwrap();

        // let res = contract
        //     .method::<(H256, Vec<Bytes>), (Vec<Bytes>, Address)>("resolve", (node, data))
        //     .unwrap()
        //     .call()
        //     .await
        //     .unwrap();

        // println!("res: {:?}", res);
        // provider.resolve_name(name).await.map_err(|e| {
        //     println!("Error resolving name: {e:?}");

        //     ProfileError::NotFound
        // })

        Err(ProfileError::NotFound)
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

        let provider = Provider::<Http>::try_from(
            "",
        )
        .unwrap();

        let res = Profile::resolve_universal("luc.eth", vec![data.to_vec(), data.to_vec()], provider).await;

        // assert_eq!(res, Err(ProfileError::NotFound));
    }
}

// 0x
// 206c74c9 // fnsig resolve(bytes node, bytes[] data)
// 0000000000000000000000000000000000000000000000000000000000000040
// 0000000000000000000000000000000000000000000000000000000000000080
// 0000000000000000000000000000000000000000000000000000000000000020
// e1e7bcf2ca33c28a806ee265cfedf02fedf1b124ca73b2203ca80cc7c91a02ad
// 0000000000000000000000000000000000000000000000000000000000000001
// 0000000000000000000000000000000000000000000000000000000000000020
// 0000000000000000000000000000000000000000000000000000000000000024
// 3b3b57de // addr()
// e1e7bcf2ca33c28a806ee265cfedf02fedf1b124ca73b2203ca80cc7c91a02ad
// 00000000000000000000000000000000000000000000000000000000


// 0x
// 0000000000000000000000000000000000000000000000000000000000000040
// 0000000000000000000000004976fb03c32e5b8cfe2b6ccb31c09ba78ebaba41
// 0000000000000000000000000000000000000000000000000000000000000001
// 0000000000000000000000000000000000000000000000000000000000000020
// 0000000000000000000000000000000000000000000000000000000000000020
// 000000000000000000000000225f137127d9067788314bc7fcc1f36746a3c3b5
