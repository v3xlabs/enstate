use worker::*;

use enstate_shared::{
    models::{multicoin::cointype::Coins, profile::Profile, records::Records},
};
use ethers::providers::{Http, Provider};
use kv_cache::CloudflareKVCache;

mod kv_cache;

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/n/:name", |_req, ctx| async move {
            if let Some(name) = ctx.param("name") {
                console_log!("name: {}", name);

                // let kv = ctx.kv("KV_CACHE").unwrap();

                let cache = Box::new(CloudflareKVCache::new());
                let profile_records = Records::default().records;
                let profile_chains = Coins::default().coins;

                let rpc = Provider::<Http>::try_from(
                    "https://rpc.enstate.rs/v1/mainnet",
                )
                .unwrap();

                match Profile::from_name(name, true, cache, rpc, &profile_records, &profile_chains)
                    .await
                {
                    Ok(data) => {
                        console_log!("data: {:?}", data);

                        return Response::from_json(&data)
                    }
                    Err(e) => {
                        console_error!("error: {}", e.to_string());
                        return Response::error(e.to_string(), 500)
                    },
                }
            }

            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
