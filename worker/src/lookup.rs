use enstate_shared::cache::CacheLayer;
use std::str::FromStr;
use std::sync::Arc;

use enstate_shared::models::multicoin::cointype::coins::CoinType;
use enstate_shared::models::profile::error::ProfileError;
use enstate_shared::models::{multicoin::cointype::Coins, profile::Profile, records::Records};
use ethers::types::Address;
use ethers::{
    providers::{Http, Provider},
    types::H160,
};
use worker::{console_log, Env, Request, Response, Url};

use crate::kv_cache::CloudflareKVCache;

pub enum LookupType {
    NameLookup(String),
    AddressLookup(String),
    NameOrAddressLookup(String),
    ImageLookup(String),
    Unknown,
}

impl From<String> for LookupType {
    fn from(path: String) -> Self {
        let split: Vec<&str> = path.split("/").filter(|it| !it.is_empty()).collect();

        if split.len() < 2 {
            return LookupType::Unknown;
        }

        let [op, arg] = split[0..2] else {
            return LookupType::Unknown;
        };

        match op {
            "n" => LookupType::NameLookup(arg.to_string()),
            "a" => LookupType::AddressLookup(arg.to_string()),
            "u" => LookupType::NameOrAddressLookup(arg.to_string()),
            "i" => LookupType::ImageLookup(arg.to_string()),
            _ => LookupType::Unknown,
        }
    }
}

impl LookupType {
    pub async fn process(
        &self,
        req: Request,
        env: Arc<Env>,
        opensea_api_key: &str,
    ) -> Result<Response, Response> {
        let cache = Box::new(CloudflareKVCache::new(env));
        let profile_records = Records::default().records;
        let profile_chains = Coins::default().coins;

        // TODO: env
        let rpc = Provider::<Http>::try_from("https://rpc.enstate.rs/v1/mainnet")
            .map_err(|_| Response::error("RPC Failure", 500).unwrap())?;

        let url = req.url().unwrap();
        let query = querystring::querify(url.query().unwrap_or(""));
        let fresh = query
            .into_iter()
            .find(|(k, v)| *k == "fresh" && *v == "true")
            .is_some();

        match self {
            LookupType::ImageLookup(name_or_address) => {
                console_log!("Avatar Lookup {}", name_or_address);

                let profile = universal_profile_resolve(
                    name_or_address,
                    fresh,
                    cache,
                    rpc,
                    &opensea_api_key,
                    &profile_records,
                    &profile_chains,
                )
                .await;

                if let Ok(profile) = profile {
                    if let Some(avatar) = profile.avatar {
                        let url = Url::parse(avatar.as_str()).unwrap();

                        return Ok(Response::redirect(url).unwrap());
                    }
                }

                Err(Response::error("Not Found", 404).unwrap())
            }
            _ => {
                let profile = match self {
                    LookupType::NameLookup(name) => {
                        console_log!("Name Lookup {}", name);

                        Profile::from_name(
                            name,
                            fresh,
                            cache,
                            rpc,
                            &opensea_api_key,
                            &profile_records,
                            &profile_chains,
                        )
                        .await
                    }
                    LookupType::AddressLookup(address) => {
                        console_log!("Address Lookup {}", address);
                        let address = H160::from_str(address).unwrap();

                        Profile::from_address(
                            address,
                            fresh,
                            cache,
                            rpc,
                            &opensea_api_key,
                            &profile_records,
                            &profile_chains,
                        )
                        .await
                    }
                    LookupType::NameOrAddressLookup(name_or_address) => {
                        console_log!("Universal Lookup {}", name_or_address);

                        universal_profile_resolve(
                            name_or_address,
                            fresh,
                            cache,
                            rpc,
                            &opensea_api_key,
                            &profile_records,
                            &profile_chains,
                        )
                        .await
                    }
                    _ => Err(ProfileError::NotFound),
                };

                // TODO: process from_json error
                profile
                    .map(|profile| Response::from_json(&profile).unwrap())
                    .map_err(|err| Response::error(err.to_string(), 404).unwrap())
            }
        }
    }
}

async fn universal_profile_resolve(
    name_or_address: &str,
    fresh: bool,
    cache: Box<dyn CacheLayer>,
    rpc: Provider<Http>,
    opensea_api_key: &str,
    profile_records: &[String],
    profile_chains: &[CoinType],
) -> Result<Profile, ProfileError> {
    let address_option: Option<Address> = name_or_address.parse().ok();

    match address_option {
        Some(address) => {
            Profile::from_address(
                address,
                fresh,
                cache,
                rpc,
                opensea_api_key,
                profile_records,
                profile_chains,
            )
            .await
        }
        None => {
            Profile::from_name(
                &name_or_address.to_lowercase(),
                fresh,
                cache,
                rpc,
                opensea_api_key,
                profile_records,
                profile_chains,
            )
            .await
        }
    }
}
