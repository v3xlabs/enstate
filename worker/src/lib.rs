use std::{ops::Deref, process, sync::Arc};

use wasm_bindgen::{JsCast, JsValue};

use enstate_shared::models::{multicoin::cointype::Coins, profile::Profile, records::Records};
use ethers::providers::{Http, Provider};
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

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    let router = Router::new();

    router
        .get_async("/n/:name", |req, ctx2: RouteContext<()>| async move {
            let ctx = Arc::new(ctx2);
            let ctx3 = ctx.clone();
            if let Some(name) = ctx3.param("name") {
                console_log!("name: {}", name);

                let x = req.url().unwrap();

                let query = x.query().unwrap_or("");
                console_log!("query: {}", query);
               
                let querys = querystring::querify(query);

                let fresh = {
                    querys.into_iter().find(|(k, _)| *k == "fresh").map(|(_, v)| v == "true").unwrap_or(false)
                };

                console_log!("fresh: {}", fresh);

                let cache = Box::new(CloudflareKVCache::new(ctx.clone()));
                let profile_records = Records::default().records;
                let profile_chains = Coins::default().coins;

                let rpc = Provider::<Http>::try_from("https://rpc.enstate.rs/v1/mainnet").unwrap();

                match Profile::from_name(name, fresh, cache, rpc, &profile_records, &profile_chains)
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

            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
