use std::rc::Rc;
use std::sync::Arc;

use enstate_shared::cache::CacheLayer;
use enstate_shared::models::profile::error::ProfileError;
use enstate_shared::models::profile::ProfileService;
use enstate_shared::models::{multicoin::cointype::Coins, records::Records};
use enstate_shared::utils::factory::SimpleFactory;
use ethers::{
    providers::{Http, Provider},
    types::H160,
};
use http::StatusCode;
use worker::{Env, Request, Response, Url};

use crate::http_util::{http_simple_status_error, profile_http_error_mapper, ErrorResponse};
use crate::kv_cache::CloudflareKVCache;

pub enum LookupType {
    NameLookup(String),
    AddressLookup(String),
    NameOrAddressLookup(String),
    ImageLookup(String),
    HeaderLookup(String),
    Unknown,
}

impl From<String> for LookupType {
    fn from(path: String) -> Self {
        let split: Vec<&str> = path.split('/').filter(|it| !it.is_empty()).collect();

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
            "h" => LookupType::HeaderLookup(arg.to_string()),
            _ => LookupType::Unknown,
        }
    }
}

impl LookupType {
    pub async fn process(
        &self,
        req: Request,
        env: Env,
        opensea_api_key: &str,
    ) -> Result<Response, Response> {
        let rc_env = Rc::new(env);

        let cache: Box<dyn CacheLayer> = Box::new(CloudflareKVCache::new(rc_env.clone()));
        let profile_records = Records::default().records;
        let profile_chains = Coins::default().coins;

        let rpc_url = rc_env
            .clone()
            .var("RPC_URL")
            .map(|x| x.to_string())
            .unwrap_or("https://rpc.enstate.rs/v1/mainnet".to_string());

        let rpc = Provider::<Http>::try_from(rpc_url)
            .map_err(|_| Response::error("RPC Failure", 500).unwrap())?;

        let url = req
            .url()
            .map_err(|_| Response::error("Worker error", 500).unwrap())?;
        let query = querystring::querify(url.query().unwrap_or(""));
        let fresh = query.into_iter().any(|(k, v)| k == "fresh" && v == "true");

        let service = ProfileService {
            cache,
            rpc: Box::new(SimpleFactory::from(Arc::new(rpc))),
            opensea_api_key: opensea_api_key.to_string(),
            profile_records: Arc::from(profile_records),
            profile_chains: Arc::from(profile_chains),
        };

        match self {
            LookupType::Unknown => Ok(Response::from_json(&ErrorResponse {
                status: 404,
                error: "Unknown route".to_string(),
            })
            .unwrap()
            .with_status(404)),
            LookupType::ImageLookup(name_or_address)
            | LookupType::HeaderLookup(name_or_address) => {
                let profile = service
                    .resolve_from_name_or_address(name_or_address, fresh)
                    .await
                    .map_err(profile_http_error_mapper)?;

                let field = match self {
                    LookupType::ImageLookup(_) => profile.avatar,
                    LookupType::HeaderLookup(_) => profile.header,
                    _ => unreachable!(),
                };

                let Some(img) = field else {
                    return Err(http_simple_status_error(StatusCode::NOT_FOUND));
                };

                let url = Url::parse(img.as_str()).map_err(|_| {
                    Response::error("Invalid avatar URL", StatusCode::NOT_ACCEPTABLE.as_u16())
                        .expect("status should be in correct range")
                })?;

                Ok(Response::redirect(url).map_err(|_| {
                    Response::error("Worker error", 500).expect("status should be in correct range")
                })?)
            }
            _ => {
                let profile = match self {
                    LookupType::NameLookup(name) => service.resolve_from_name(name, fresh).await,
                    LookupType::AddressLookup(address) => {
                        let address = address.parse::<H160>();

                        match address {
                            Ok(address) => service.resolve_from_address(address, fresh).await,
                            Err(_) => Err(ProfileError::NotFound),
                        }
                    }
                    LookupType::NameOrAddressLookup(name_or_address) => {
                        service
                            .resolve_from_name_or_address(name_or_address, fresh)
                            .await
                    }
                    _ => unreachable!(),
                }
                .map_err(profile_http_error_mapper)?;

                Response::from_json(&profile)
                    .map_err(|_| http_simple_status_error(StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
    }
}
