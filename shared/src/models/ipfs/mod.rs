use std::env;

use tracing::info;

use super::erc721::metadata::NFTMetadata;

#[derive(Debug, PartialEq)]
pub enum IPFSURLUnparsed {
    URL(String),
    IPFS(String),
    // IPNS(String),
}

impl IPFSURLUnparsed {
    // Given an arbitrary value initializes the ipfsurlunparsed
    pub fn from_unparsed(value: String) -> Self {
        let raw_ipfs = regex::Regex::new(r"^Qm[1-9A-HJ-NP-Za-km-z]{44,}|b[A-Za-z2-7]{58,}|B[A-Z2-7]{58,}|z[1-9A-HJ-NP-Za-km-z]{48,}|F[0-9A-F]{50,}$").unwrap();
        if raw_ipfs.is_match(&value) {
            return IPFSURLUnparsed::IPFS(value);
        }

        // If IPFS
        let ipfs = regex::Regex::new(r"^ipfs://(ip[fn]s/)?([0-9a-zA-Z]+(/.*)?)").unwrap();
        if let Some(captures) = ipfs.captures(&value) {
            let hash = captures.get(2).unwrap().as_str();

            return IPFSURLUnparsed::IPFS(hash.to_string());
        }

        IPFSURLUnparsed::URL(value)
    }

    pub fn from_ipfs(value: String) -> Self {
        Self::from_unparsed(value)
    }

    // This function turns the unparsed
    pub fn to_url_or_gateway(&self) -> String {
        match self {
            IPFSURLUnparsed::URL(url) => url.to_string(),
            IPFSURLUnparsed::IPFS(hash) => format!("https://ipfs.io/ipfs/{}", hash),
        }
    }

    pub async fn fetch(&self, opensea_api_key: &str) -> Result<NFTMetadata, ()> {
        let url = self.to_url_or_gateway();
        let mut client_headers = reqwest::header::HeaderMap::new();

        if url.starts_with("https://api.opensea.io/") {
            client_headers.insert(
                "X-API-KEY",
                opensea_api_key.parse().unwrap(),
            );
        }

        let client = reqwest::Client::builder()
            .default_headers(client_headers)
            .build()
            .unwrap();

        let res = client.get(&url).send().await.unwrap();

        let body = res.text().await.unwrap();

        let metadata: NFTMetadata = serde_json::from_str(&body).unwrap();

        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        // leontalbert.eth
        assert_eq!(
            IPFSURLUnparsed::from_unparsed(
                "QmVzke12sVaUANLBqdrLcCWtzy87bW8HVC92QjdEqyZYCq".to_string()
            ),
            IPFSURLUnparsed::IPFS("QmVzke12sVaUANLBqdrLcCWtzy87bW8HVC92QjdEqyZYCq".to_string())
        );
        // poap.eth
        assert_eq!(
            IPFSURLUnparsed::from_unparsed(
                "ipfs://QmciEfu55sxxFx6XxXpF2wwzx6PfimpmyffYQgBJzF7pAM".to_string()
            ),
            IPFSURLUnparsed::IPFS("QmciEfu55sxxFx6XxXpF2wwzx6PfimpmyffYQgBJzF7pAM".to_string())
        );
        // pedrouid.eth
        assert_eq!(
            IPFSURLUnparsed::from_unparsed(
                "ipfs://ipfs/QmY5R64EkwZ7ru6Nbk2neTV8RxrMGE4LSF8h3xE4CGQttH/image.jpeg".to_string()
            ),
            IPFSURLUnparsed::IPFS(
                "QmY5R64EkwZ7ru6Nbk2neTV8RxrMGE4LSF8h3xE4CGQttH/image.jpeg".to_string()
            )
        );
    }

    #[tokio::test]
    async fn test_ipfs_url_unparsed() {
        let url = IPFSURLUnparsed::from_unparsed("https://creature.mypinata.cloud/ipfs/QmVDNzQNuD5jBKHmJ2nmVP35HsXUqhGRX9V2KVHvRznLg8/2257".to_string());
        let opensea_api_key = env::var("OPENSEA_API_KEY").unwrap().to_string();

        let result = url.fetch(&opensea_api_key).await.unwrap();

        assert_eq!(result.name.unwrap(), "Creature #2257");
        assert_eq!(result.image.unwrap(), "https://creature.mypinata.cloud/ipfs/QmeZGc1CL3eb9QJatKXTGT7ekgLMq9FyZUWckQ4oWdc53a/2257.jpg");
    }

    #[tokio::test]
    async fn test_ipfs_url() {
        let url = IPFSURLUnparsed::URL("https://api.opensea.io/api/v1/metadata/0x495f947276749Ce646f68AC8c248420045cb7b5e/20709508835757291459772958604787444705400082683953919595999414934333676322817".to_string());
        let opensea_api_key = env::var("OPENSEA_API_KEY").unwrap().to_string();

        let result = url.fetch(&opensea_api_key).await.unwrap();

        assert_eq!(result.name.unwrap(), "choob");
    }
}
