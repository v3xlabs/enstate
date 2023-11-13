use std::{collections::VecDeque, str::FromStr};
use std::sync::Arc;

use wasm_bindgen::JsValue;

use enstate_shared::models::{
    multicoin::cointype::Coins,
    profile::{Profile},
    records::Records,
};
use ethers::{
    providers::{Http, Provider},
    types::H160,
};
use ethers::types::Address;
use js_sys::Reflect;
use kv_cache::CloudflareKVCache;
use worker::{
    console_error, console_log, event, Context, Cors, Env, Method, Request, Response, Url,
};

mod kv_cache;

fn get_js(target: &JsValue, name: &str) -> Result<JsValue, JsValue> {
    Reflect::get(target, &JsValue::from(name))
}

pub enum LookupType {
    NameLookup(String),
    AddressLookup(String),
    NameOrAddressLookup(String),
    ImageLookup(String),
    Unknown,
}

impl LookupType {
    fn from_path(path: String) -> Self {
        let mut split: VecDeque<&str> = path.split("/").collect::<Vec<&str>>().into();

        let _ = split.pop_front();
        let first = split.pop_front().unwrap_or("");

        match first {
            "n" => {
                if let Some(name) = split.pop_front() {
                    return LookupType::NameLookup(name.to_string());
                }
                LookupType::Unknown
            }
            "a" => {
                if let Some(address) = split.pop_front() {
                    return LookupType::AddressLookup(address.to_string());
                }
                LookupType::Unknown
            }
            "u" => {
                if let Some(name_or_address) = split.pop_front() {
                    return LookupType::NameOrAddressLookup(name_or_address.to_string());
                }
                LookupType::Unknown
            }
            "i" => {
                if let Some(name) = split.pop_front() {
                    return LookupType::ImageLookup(name.to_string());
                }
                LookupType::Unknown
            }
            _ => return LookupType::Unknown,
        }
    }

    async fn process(&self, req: Request, env: Arc<Env>, opensea_api_key: &str) -> Result<Response, Response> {
        let cache = Box::new(CloudflareKVCache::new(env));
        let profile_records = Records::default().records;
        let profile_chains = Coins::default().coins;
        let rpc = Provider::<Http>::try_from("https://rpc.enstate.rs/v1/mainnet")
            .map_err(|_| Response::error("RPC Failure", 500).unwrap())?;

        let url = req.url().unwrap();
        let query = querystring::querify(url.query().unwrap_or(""));
        let fresh = query.into_iter().find(|(k, _)| *k == "fresh").is_some();

        match self {
            LookupType::NameLookup(name) => {
                console_log!("Name Lookup {}", name);

                let profile = Profile::from_name(
                    name.as_str(),
                    fresh,
                    cache,
                    rpc,
                    &opensea_api_key,
                    &profile_records,
                    &profile_chains,
                )
                .await;

                match profile {
                    Ok(x) => Ok(Response::from_json(&x).map_err(|e| {
                        console_error!("error: {}", e.to_string());
                        Response::error(e.to_string(), 404).unwrap()
                    })?),
                    Err(e) => {
                        console_error!("error: {}", e.to_string());
                        return Err(Response::error(e.to_string(), 404).unwrap());
                    }
                }
            }
            LookupType::AddressLookup(address) => {
                console_log!("Address Lookup {}", address);
                let address = H160::from_str(address.as_str()).unwrap();

                let profile = Profile::from_address(
                    address,
                    fresh,
                    cache,
                    rpc,
                    &opensea_api_key,
                    &profile_records,
                    &profile_chains,
                )
                .await;

                match profile {
                    Ok(x) => Ok(Response::from_json(&x).unwrap()),
                    Err(e) => {
                        console_error!("error: {}", e.to_string());
                        return Err(Response::error(e.to_string(), 404).unwrap());
                    }
                }
            },
            LookupType::NameOrAddressLookup(name_or_address) => {
                console_log!("Universal Lookup {}", name_or_address);

                let address_option: Option<Address> = name_or_address.parse().ok();

                let profile = match address_option {
                    Some(address) => {
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
                    None => {
                        Profile::from_name(
                            &name_or_address.to_lowercase(),
                            fresh,
                            cache,
                            rpc,
                            &opensea_api_key,
                            &profile_records,
                            &profile_chains,
                        )
                            .await
                    }
                };

                match profile {
                    Ok(x) => Ok(Response::from_json(&x).map_err(|e| {
                        console_error!("error: {}", e.to_string());
                        Response::error(e.to_string(), 404).unwrap()
                    })?),
                    Err(e) => {
                        console_error!("error: {}", e.to_string());
                        return Err(Response::error(e.to_string(), 404).unwrap());
                    }
                }
            }
            LookupType::ImageLookup(name) => {
                console_log!("Avatar Lookup {}", name);

                let profile = Profile::from_name(
                    name.as_str(),
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
                console_log!("Unknown Lookup");
                Err(Response::error("Unknown Lookup", 501).unwrap())
            }
        }
    }
}

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    let cors = Cors::default()
        .with_origins(vec!["*"])
        .with_methods(Method::all());

    let opensea_api_key = env.var("OPENSEA_API_KEY").unwrap().to_string();

    let env_arc = Arc::new(env);

    let response = LookupType::from_path(req.path())
        .process(req, env_arc, &opensea_api_key)
        .await
        .unwrap_or_else(|f| f);

    let mut headers = response.headers().clone();

    let _ = headers.set("Cache-Control", "max-age=600, stale-while-revalidate=30");

    response.with_headers(headers).with_cors(&cors)
}
