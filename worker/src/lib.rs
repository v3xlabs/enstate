use std::{collections::VecDeque, str::FromStr, sync::Arc};

use wasm_bindgen::JsValue;

use enstate_shared::models::{multicoin::cointype::Coins, profile::Profile, records::Records};
use ethers::{
    providers::{Http, Provider},
    types::H160,
};
use js_sys::Reflect;
use kv_cache::CloudflareKVCache;
use worker::{console_error, console_log, event, Context, Cors, Env, Method, Request, Response, Url};

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

        console_log!("first: {}, path {}", first, path);

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
}

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    let env = Arc::new(env);
    let cache = Box::new(CloudflareKVCache::new(env));
    let profile_records = Records::default().records;
    let profile_chains = Coins::default().coins;
    let rpc = Provider::<Http>::try_from("https://rpc.enstate.rs/v1/mainnet").unwrap();

    let url = req.url().unwrap();
    let query = querystring::querify(url.query().unwrap_or(""));
    let fresh = query.into_iter().find(|(k, _)| *k == "fresh").is_some();

    let cors = Cors::default()
        .with_origins(vec!["*"])
        .with_methods(Method::all());

    match LookupType::from_path(req.path()) {
        LookupType::NameLookup(name) => {
            console_log!("Name Lookup {}", name);

            match Profile::from_name(
                name.as_str(),
                fresh,
                cache,
                rpc,
                &profile_records,
                &profile_chains,
            )
            .await
            {
                Ok(data) => {
                    console_log!("data: {:?}", data);

                    return Response::from_json(&data);
                }
                Err(e) => {
                    console_error!("error: {}", e.to_string());
                    return Response::error(e.to_string(), 500);
                }
            }
        }
        LookupType::AddressLookup(address) => {
            console_log!("Address Lookup {}", address);
            let address = H160::from_str(address.as_str()).unwrap();

            Profile::from_address(
                address,
                fresh,
                cache,
                rpc,
                &profile_records,
                &profile_chains,
            )
            .await
            .map(|x| Response::from_json(&x))
            .map_err(|e| {
                console_error!("error: {}", e.to_string());
                return Response::error(e.to_string(), 500);
            })
            .unwrap()
        }
        LookupType::ImageLookup(name) => {
            console_log!("Avatar Lookup {}", name);

            let profile = Profile::from_name(
                name.as_str(),
                fresh,
                cache,
                rpc,
                &profile_records,
                &profile_chains,
            )
            .await;

            if let Ok(profile) = profile {
                if let Some(avatar) = profile.avatar {
                    let url = Url::parse(avatar.as_str()).unwrap();

                    return Response::redirect(url);
                }
            }

            Response::error("Not Found", 404)
        }
        _ => {
            console_log!("Unknown Lookup");
            Response::error("Unknown Lookup", 501)
        }
    }
    .map(|x| x.with_cors(&cors).unwrap())
}
