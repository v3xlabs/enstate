use super::{ENSLookup, ENSLookupError};

use ethers_core::{
    abi::{ParamType, Token},
    types::H256,
};
use hex_literal::hex;
use tracing::info;

pub struct Avatar {
    pub ipfs_gateway: String,
    pub name: String,
}

impl Avatar {}

impl ENSLookup for Avatar {
    fn calldata(&self, namehash: &H256) -> Vec<u8> {
        let fn_selector = hex!("59d1d43c").to_vec();

        let data = ethers_core::abi::encode(&[
            Token::FixedBytes(namehash.as_fixed_bytes().to_vec()),
            Token::String("avatar".to_string()),
        ]);

        [fn_selector, data].concat()
    }

    fn decode(&self, data: &[u8]) -> Result<String, ENSLookupError> {
        let decoded_abi = ethers_core::abi::decode(&[ParamType::String], data)
            .map_err(|_| ENSLookupError::AbiDecodeError)?;
        let value = decoded_abi.get(0).ok_or(ENSLookupError::AbiDecodeError)?;
        let value = value.to_string();

        // If IPFS
        let ipfs = regex::Regex::new(r"ipfs://([0-9a-zA-Z]+)").unwrap();
        if let Some(captures) = ipfs.captures(&value) {
            let hash = captures.get(1).unwrap().as_str();

            return Ok(format!("{}{hash}", self.ipfs_gateway));
        }

        // If the raw value is eip155 url
        let eip155 =
            regex::Regex::new(r"eip155:([0-9]+)/(erc1155|erc721):0x([0-9a-fA-F]{40})/([0-9]+)")
                .unwrap();

        if let Some(captures) = eip155.captures(&value) {
            let chain_id = captures.get(1).unwrap().as_str();
            let contract_type = captures.get(2).unwrap().as_str();
            let contract_address = captures.get(3).unwrap().as_str();
            let token_id = captures.get(4).unwrap().as_str();

            info!(
                "Encountered Avatar: {chain_id} {contract_type} {contract_address} {token_id}",
                chain_id = chain_id,
                contract_type = contract_type,
                contract_address = contract_address,
                token_id = token_id
            );

            // TODO: Remove naive approach
            return Ok(format!(
                "https://metadata.ens.domains/mainnet/avatar/{}",
                self.name
            ));
        }

        Ok(value)
    }

    fn name(&self) -> String {
        "avatar".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::providers::namehash;

    #[tokio::test]
    async fn test_calldata_avatar() {
        assert_eq!(
            Avatar{
                ipfs_gateway: "https://ipfs.io/ipfs/".to_string(),
                name: "luc.eth".to_string(),
            }.calldata(&namehash("luc.eth")),
            hex_literal::hex!("59d1d43ce1e7bcf2ca33c28a806ee265cfedf02fedf1b124ca73b2203ca80cc7c91a02ad000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000066176617461720000000000000000000000000000000000000000000000000000")
        );
    }
}
