use std::time::Duration;

use data_url::{DataUrl, DataUrlError};
use lazy_static::lazy_static;
use reqwest::header::HeaderValue;
use thiserror::Error;

use crate::models::erc721::metadata::NFTMetadata;
use crate::models::lookup::LookupState;

#[derive(Debug, PartialEq)]
pub enum URLUnparsed {
    HTTP { url: String },
    IPFS { path: String },
    Arweave { hash: String },
    Data { url: String },
    // IPNS(String),
}

#[derive(Debug, Error)]
pub enum URLParseError {
    #[error("URL parse error: {0}")]
    URLParseError(#[from] url::ParseError),

    #[error("Invalid IPFS URL: {0}")]
    InvalidIPFSUrl(String),

    #[error("Unsupported protocol: {0}")]
    UnsupportedProtocol(String),
}

#[derive(Debug, Error)]
pub enum URLFetchError {
    #[error("HTTP error: {0}")]
    HTTPError(#[from] reqwest::Error),

    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Data URL Parse error: {0}")]
    DataURLParseError(#[from] DataUrlError),

    #[error("Data URL Base64 error")]
    DataURLBase64Error,
}

pub const OPENSEA_BASE_PREFIX: &str = "https://api.opensea.io/";

lazy_static! {
    static ref RAW_IPFS_REGEX: regex::Regex =
        regex::Regex::new(r"^(?:Qm[1-9A-HJ-NP-Za-km-z]{44,}|b[A-Za-z2-7]{58,}|B[A-Z2-7]{58,}|z[1-9A-HJ-NP-Za-km-z]{48,}|F[0-9A-F]{50,})$")
            .expect("should be a valid regex");

    static ref LEADING_SLASH_REGEX: regex::Regex = regex::Regex::new(r"^\/+").expect("should be a valid regex");
}

impl URLUnparsed {
    pub fn from_unparsed(value: &str) -> Result<Self, URLParseError> {
        if RAW_IPFS_REGEX.is_match(value) {
            return Ok(URLUnparsed::IPFS {
                path: value.to_string(),
            });
        }

        let parsed = url::Url::parse(value)?;

        match parsed.scheme() {
            "ipfs" => {
                let hash = match parsed.domain() {
                    Some(it) if it.to_lowercase() == "ipfs" => {
                        if parsed.path().len() <= 1 {
                            return Err(URLParseError::InvalidIPFSUrl(value.to_string()));
                        };

                        Ok(LEADING_SLASH_REGEX
                            .replace_all(parsed.path(), "")
                            .to_string())
                    }
                    Some(it) => Ok(it.to_string() + parsed.path()),
                    None => Err(URLParseError::InvalidIPFSUrl(value.to_string())),
                }?;

                Ok(URLUnparsed::IPFS { path: hash })
            }
            "ar" => Ok(URLUnparsed::Arweave {
                hash: parsed.path().to_string(),
            }),
            "http" | "https" => Ok(URLUnparsed::HTTP {
                url: value.to_string(),
            }),
            "data" => Ok(URLUnparsed::Data {
                url: value.to_string(),
            }),
            other => Err(URLParseError::UnsupportedProtocol(other.to_string())),
        }
    }

    // This function turns the unparsed
    pub fn to_url_or_ipfs_gateway(&self, state: &LookupState) -> String {
        match self {
            URLUnparsed::HTTP { url } | URLUnparsed::Data { url } => url.to_string(),
            URLUnparsed::IPFS { path } => {
                format!("{gateway}{path}", gateway = state.ipfs_gateway)
            }
            URLUnparsed::Arweave { hash } => {
                format!("{gateway}{hash}", gateway = state.arweave_gateway)
            }
        }
    }

    pub async fn fetch(&self, state: &LookupState) -> Result<NFTMetadata, URLFetchError> {
        let metadata_json = if let URLUnparsed::Data { url } = self {
            let data_url = DataUrl::process(url)?;

            let (body, _) = data_url
                .decode_to_vec()
                .map_err(|_| URLFetchError::DataURLBase64Error)?;

            String::from_utf8_lossy(&body).to_string()
        } else {
            let url = self.to_url_or_ipfs_gateway(state);
            let mut client_headers = reqwest::header::HeaderMap::new();

            if url.starts_with(OPENSEA_BASE_PREFIX) {
                client_headers.insert(
                    "X-API-KEY",
                    HeaderValue::from_str(&state.opensea_api_key)
                        .unwrap_or_else(|_| HeaderValue::from_static("")),
                );
            }

            let client = reqwest::Client::builder()
                .default_headers(client_headers)
                .timeout(Duration::from_secs(4))
                .build()?;

            let res = client.get(&url).send().await?;

            res.text().await?
        };

        let metadata: NFTMetadata = serde_json::from_str(&metadata_json)?;

        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ipfs() {
        // leontalbert.eth
        assert_eq!(
            URLUnparsed::from_unparsed("QmVzke12sVaUANLBqdrLcCWtzy87bW8HVC92QjdEqyZYCq").unwrap(),
            URLUnparsed::IPFS {
                path: "QmVzke12sVaUANLBqdrLcCWtzy87bW8HVC92QjdEqyZYCq".to_string()
            }
        );
        // poap.eth
        assert_eq!(
            URLUnparsed::from_unparsed("ipfs://QmciEfu55sxxFx6XxXpF2wwzx6PfimpmyffYQgBJzF7pAM")
                .unwrap(),
            URLUnparsed::IPFS {
                path: "QmciEfu55sxxFx6XxXpF2wwzx6PfimpmyffYQgBJzF7pAM".to_string()
            }
        );
        assert_eq!(
            URLUnparsed::from_unparsed(
                "ipfs://QmY5R64EkwZ7ru6Nbk2neTV8RxrMGE4LSF8h3xE4CGQttH/image.jpeg"
            )
            .unwrap(),
            URLUnparsed::IPFS {
                path: "QmY5R64EkwZ7ru6Nbk2neTV8RxrMGE4LSF8h3xE4CGQttH/image.jpeg".to_string()
            }
        );
        assert_eq!(
            URLUnparsed::from_unparsed(
                "ipfs://ipfs/QmY5R64EkwZ7ru6Nbk2neTV8RxrMGE4LSF8h3xE4CGQttH/image.jpeg"
            )
            .unwrap(),
            URLUnparsed::IPFS {
                path: "QmY5R64EkwZ7ru6Nbk2neTV8RxrMGE4LSF8h3xE4CGQttH/image.jpeg".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_http() {
        assert_eq!(
            URLUnparsed::from_unparsed("https://id.antony.cloud/img/2.png").unwrap(),
            URLUnparsed::HTTP {
                url: "https://id.antony.cloud/img/2.png".to_string()
            }
        );
        assert_eq!(
            URLUnparsed::from_unparsed("http://id.antony.cloud/img/2.png").unwrap(),
            URLUnparsed::HTTP {
                url: "http://id.antony.cloud/img/2.png".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_data() {
        assert_eq!(
            URLUnparsed::from_unparsed("data:application/json;base64,e30=").unwrap(),
            URLUnparsed::Data {
                url: "data:application/json;base64,e30=".to_string()
            }
        )
    }
}
