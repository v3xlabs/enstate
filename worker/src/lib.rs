use wasm_bindgen::{JsCast, JsValue};

use enstate_shared::models::{multicoin::cointype::Coins, profile::Profile, records::Records};
use ethers::providers::{Http, Provider};
use js_sys::{global, Function, Object, Promise, Reflect, Uint8Array};
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
        .get_async("/n/:name", |_req, ctx: RouteContext<()>| async move {
            if let Some(name) = ctx.param("name") {
                console_log!("name: {}", name);

                let kvStore = getJS(&ctx.env, "enstate-1").unwrap();

                console_log!("xx: {:?}", kvStore);

                let getFunctionValue = getJS(&kvStore, "get").unwrap();

                let getFunction = getFunctionValue.dyn_into::<Function>().unwrap();

                console_log!("xz: {:?}", getFunction);
                let options = JsValue::default();

                let getFunctionPromise: Promise = getFunction
                    .call2(
                        &kvStore,
                        &JsValue::from_str("test"),
                        &options,
                    )
                    .unwrap()
                    .into();

                let getFunctionResult = JsFuture::from(getFunctionPromise).await.unwrap();

                console_log!("pxxxz: {:?}", getFunctionResult);

                let putFunctionValue = getJS(&kvStore, "put").unwrap();

                let putFunction = putFunctionValue.dyn_into::<Function>().unwrap();

                console_log!("pxz: {:?}", putFunction);
                let options = JsValue::default();

                let putFunctionPromise: Promise = putFunction
                    .call3(
                        &kvStore,
                        &JsValue::from_str("test"),
                        &JsValue::from_str("data-lol"),
                        &options,
                    )
                    .unwrap()
                    .into();

                let putFunctionResult = JsFuture::from(putFunctionPromise).await.unwrap();

                console_log!("pxxxz: {:?}", putFunctionResult);

                let cache = Box::new(CloudflareKVCache::new());
                let profile_records = Records::default().records;
                let profile_chains = Coins::default().coins;

                let rpc = Provider::<Http>::try_from("https://rpc.enstate.rs/v1/mainnet").unwrap();

                match Profile::from_name(name, false, cache, rpc, &profile_records, &profile_chains)
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
