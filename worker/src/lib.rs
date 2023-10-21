use worker::*;

use enstate_shared::{cache::CacheLayer, models::{profile::Profile, multicoin::cointype::Coins, records::Records}};
use ethers::providers::{Http, Middleware, Provider};
use kv_cache::CloudflareKVCache;

mod kv_cache;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let router = Router::new();
    let rpc = Provider::<Http>::try_from(
        "https://rpc.ankr.com/eth/17c7bd60d262bc06008f0a111ca740955ee09d4bb33ecb57d2700214c8f625f1",
    )
    .unwrap();

    // let kv = env.kv("enstate").unwrap();

    let cache = Box::new(CloudflareKVCache::new());
    let profile_records = Records::default().records;
    let profile_chains = Coins::default().coins;

    router
        .get_async("/n/:name", |_req, ctx| async move {
            if let Some(name) = ctx.param("name") {
                println!("name: {}", name);

                // match Profile::from_name(name, false, cache, rpc, profile_records, profile_chains)
                //     .await
                // {
                //     Ok(data) => {
                //         // let json = serde_json::to_string(&data).unwrap();

                //         // Response::from_json(json).unwrap()
                //         return Response::ok("Hello World");
                //     }
                //     Err(e) => Response::error(e.to_string(), 500),
                // }
                return Response::ok("Hello World");
            }

            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
