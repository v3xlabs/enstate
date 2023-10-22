use std::{collections::VecDeque, ops::Deref, process, str::FromStr, sync::Arc};

use wasm_bindgen::{JsCast, JsValue};

use enstate_shared::models::{multicoin::cointype::Coins, profile::Profile, records::Records};
use ethers::{
    providers::{Http, Provider},
    types::H160,
};
use js_sys::{global, Array, Function, Object, Promise, Reflect, Uint8Array};
use kv_cache::CloudflareKVCache;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use worker::{
    console_error, console_log, event, Context, Env, Request, Response, RouteContext, Router,
};

mod kv_cache;

fn getJS(target: &JsValue, name: &str) -> Result<JsValue, JsValue> {
    Reflect::get(target, &JsValue::from(name))
}

pub enum LookupType {
    NameLookup(String),
    AddressLookup(String),
    NameOrAddressLookup(String),
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

    match LookupType::from_path(req.path()) {
        LookupType::NameLookup(name) => {
            console_log!("Name Lookup {}", name);

            let x = req.url().unwrap();

            let query = x.query().unwrap_or("");
            console_log!("query: {}", query);

            let querys = querystring::querify(query);

            let fresh = {
                querys
                    .into_iter()
                    .find(|(k, _)| *k == "fresh")
                    .map(|(_, v)| v == "true")
                    .unwrap_or(false)
            };

            console_log!("fresh: {}", fresh);

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

            let x = req.url().unwrap();
            let query = x.query().unwrap_or("");
            console_log!("query: {}", query);

            let querys = querystring::querify(query);

            let fresh = {
                querys
                    .into_iter()
                    .find(|(k, _)| *k == "fresh")
                    .map(|(_, v)| v == "true")
                    .unwrap_or(false)
            };

            console_log!("fresh: {}", fresh);

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
            }).unwrap()
        }
        _ => {
            console_log!("Unknown Lookup");
            Response::error("Unknown Lookup", 501)
        }
    }
}
