use tracing::info;

use super::erc721::metadata::NFTMetadata;

pub enum IPFSURLUnparsed {
    URL(String),
    IPFS(String),
    // IPNS(String),
}

impl IPFSURLUnparsed {
    // Given an arbitrary value initializes the ipfsurlunparsed
    pub fn from_unparsed(value: String) -> Self {
        // TODO: Parse IPFS
        IPFSURLUnparsed::URL(value)
    }

    // This function turns the unparsed
    pub fn to_url_or_gateway(&self) -> String {
        match self {
            IPFSURLUnparsed::URL(url) => url.to_string(),
            IPFSURLUnparsed::IPFS(hash) => format!("https://ipfs.io/ipfs/{}", hash),
        }
    }

    pub async fn fetch(&self) -> Result<NFTMetadata, ()> {
        let url = self.to_url_or_gateway();
        let res = reqwest::get(&url).await.unwrap();

        info!("Status: {}", res.status());
        
        let body = res.text().await.unwrap();

        info!("Body:\n\n{}", body);

        let metadata: NFTMetadata = serde_json::from_str(&body).unwrap();

        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ipfs_url_unparsed() {
        let url = IPFSURLUnparsed::from_unparsed("https://creature.mypinata.cloud/ipfs/QmVDNzQNuD5jBKHmJ2nmVP35HsXUqhGRX9V2KVHvRznLg8/2257".to_string());

        let result = url.fetch().await.unwrap();

        assert_eq!(result.name, "Creature #2257");
        assert_eq!(result.image, "https://creature.mypinata.cloud/ipfs/QmeZGc1CL3eb9QJatKXTGT7ekgLMq9FyZUWckQ4oWdc53a/2257.jpg");
    }
}
