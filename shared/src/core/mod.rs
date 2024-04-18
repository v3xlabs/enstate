use std::collections::BTreeMap;
use std::sync::Arc;

use ethers::prelude::Http;
use ethers::providers::Provider;
use ethers_ccip_read::CCIPReadMiddleware;
use ethers_core::types::H160;
use serde::{Deserialize, Serialize};

use crate::models::multicoin::cointype::coins::CoinType;
use crate::utils::eip55::EIP55Address;
use crate::utils::factory::Factory;

pub mod address;
pub mod error;
pub mod lookup_data;
pub mod profile;
pub mod records;
pub mod resolvers;

pub type CCIPProvider = CCIPReadMiddleware<Arc<Provider<Http>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    // Name
    pub name: String,
    // Ethereum Mainnet Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<EIP55Address>,
    // Avatar URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    // Header URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    // Preferred Capitalization of Name
    pub display: String,
    // Content Hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contenthash: Option<String>,
    // Records
    pub records: BTreeMap<String, String>,
    // Addresses on different chains
    pub chains: BTreeMap<String, String>,
    // Unix Timestamp of date it was loaded
    pub fresh: i64,
    // Resolver the information was fetched from
    pub resolver: EIP55Address,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub ccip_urls: Vec<String>,
    // Errors encountered while fetching & decoding
    pub errors: BTreeMap<String, String>,
}

pub struct ENSService {
    pub cache: Box<dyn crate::cache::CacheLayer>,
    pub rpc: Box<dyn Factory<Arc<Provider<Http>>>>,
    pub opensea_api_key: String,
    pub ipfs_gateway: String,
    pub arweave_gateway: String,
    pub profile_records: Arc<[String]>,
    pub profile_chains: Arc<[CoinType]>,
    pub universal_resolver: H160,
    pub max_bulk_size: Option<usize>,
    pub cache_ttl: Option<u32>,
}
