use std::convert::Infallible;

use chrono::format;
use ethers::types::H256;

use crate::models::profile::Profile;

impl Profile {
    pub fn calldata_avatar(namehash: &H256) -> Vec<u8> {
        Self::calldata_text(namehash, "avatar")
    }

    pub async fn decode_avatar(name: &str, data: &[u8]) -> Result<String, Infallible> {
        let raw_value = Self::decode_text(data).unwrap();

        let ipfs = regex::Regex::new(r"ipfs://([0-9a-zA-Z]+)").unwrap();

        if let Some(_captures) = ipfs.captures(&raw_value) {
            let hash = _captures.get(1).unwrap().as_str();

            return Ok(format!("https://ipfs.io/ipfs/{}", hash));
        }

        // If the raw value is eip155 url
        let eip155 =
            regex::Regex::new(r"eip155:([0-9]+)/(erc1155|erc712):0x([0-9a-fA-F]{40})/([0-9]+)")
                .unwrap();

        if let Some(_captures) = eip155.captures(&raw_value) {
            // TODO: Remove naive approach
            return Ok(format!("https://metadata.ens.domains/mainnet/avatar/{}", name));

            // let chain_id = captures.get(1).unwrap().as_str();
            // let contract_type = captures.get(2).unwrap().as_str();
            // let contract_address = captures.get(3).unwrap().as_str();
            // let token_id = captures.get(4).unwrap().as_str();

            // let url = match contract_type {
            //     "erc1155" => format!(
            //         "https://api.opensea.io/asset/{}/{}",
            //         contract_address, token_id
            //     ),
            //     "erc712" => format!(
            //         "https://api.opensea.io/asset/{}/{}",
            //         contract_address, token_id
            //     ),
            //     _ => format!(
            //         "https://api.opensea.io/asset/{}/{}",
            //         contract_address, token_id
            //     ),
            // };

            // return Ok(url);
        }

        Ok(raw_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::providers::namehash;

    #[tokio::test]
    async fn test_calldata_avatar() {
        assert_eq!(
            hex::encode(Profile::calldata_avatar(&namehash("luc.eth"))),
            "59d1d43ce1e7bcf2ca33c28a806ee265cfedf02fedf1b124ca73b2203ca80cc7c91a02ad000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000066176617461720000000000000000000000000000000000000000000000000000"
        );
    }
}
