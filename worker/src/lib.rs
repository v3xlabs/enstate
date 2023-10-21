use worker::*;

use enstate_shared::{
    cache::CacheLayer,
    models::{multicoin::cointype::Coins, profile::Profile, records::Records},
};
use ethers::providers::{Http, Middleware, Provider};
use kv_cache::CloudflareKVCache;

mod kv_cache;

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/n/:name", |_req, ctx| async move {
            if let Some(name) = ctx.param("name") {
                console_log!("name: {}", name);
                let cache = Box::new(CloudflareKVCache::new());
                let profile_records = Records::default().records;
                let profile_chains = Coins::default().coins;

                let rpc = Provider::<Http>::try_from(
                    "https://rpc.ankr.com/eth/17c7bd60d262bc06008f0a111ca740955ee09d4bb33ecb57d2700214c8f625f1",
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
    // Response::ok("Hello World")
}
