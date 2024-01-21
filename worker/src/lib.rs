use std::sync::Arc;

use enstate_shared::cache::CacheLayer;
use enstate_shared::core::ENSService;
use enstate_shared::meta::gen_app_meta;
use enstate_shared::models::multicoin::cointype::Coins;
use enstate_shared::models::records::Records;
use enstate_shared::utils::factory::SimpleFactory;
use ethers::prelude::{Http, Provider};
use ethers::types::H160;
use http::StatusCode;
use lazy_static::lazy_static;
use worker::{event, Context, Cors, Env, Headers, Method, Request, Response, Router};

use crate::http_util::http_simple_status_error;
use crate::kv_cache::CloudflareKVCache;

mod bulk_util;
mod http_util;
mod kv_cache;
mod routes;

lazy_static! {
    static ref CORS: Cors = Cors::default()
        .with_origins(vec!["*"])
        .with_methods(Method::all());
}

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    let opensea_api_key = env
        .var("OPENSEA_API_KEY")
        .expect("OPENSEA_API_KEY should've been set")
        .to_string();

    let cache: Box<dyn CacheLayer> = Box::new(CloudflareKVCache {
        env: Env::from(env.clone()),
    });
    let profile_records = Records::default().records;
    let profile_chains = Coins::default().coins;

    let rpc_url = env
        .var("RPC_URL")
        .map(|x| x.to_string())
        .unwrap_or("https://rpc.enstate.rs/v1/mainnet".to_string());

    let rpc = Provider::<Http>::try_from(rpc_url)
        .map_err(|_| http_simple_status_error(StatusCode::BAD_REQUEST))?;

    let universal_resolver = env
        .var("UNIVERSAL_RESOLVER")
        .expect("UNIVERSAL_RESOLVER should've been set")
        .to_string()
        .parse::<H160>()
        .expect("UNIVERSAL_RESOLVER should be a valid address");

    let service = ENSService {
        cache,
        rpc: Box::new(SimpleFactory::from(Arc::new(rpc))),
        opensea_api_key: opensea_api_key.to_string(),
        profile_records: Arc::from(profile_records),
        profile_chains: Arc::from(profile_chains),
        universal_resolver,
    };

    let response = Router::with_data(service)
        .get("/", |_, _| root_handler().with_cors(&CORS))
        .get_async("/a/:address", routes::address::get)
        .get_async("/n/:name", routes::name::get)
        .get_async("/u/:name_or_address", routes::universal::get)
        .get_async("/i/:name_or_address", routes::image::get)
        .get_async("/h/:name_or_address", routes::header::get)
        .get_async("/bulk/a", routes::address::get_bulk)
        .get_async("/bulk/n", routes::name::get_bulk)
        .get_async("/bulk/u", routes::universal::get_bulk)
        .run(req, env)
        .await
        .and_then(|response| response.with_cors(&CORS));

    if let Err(err) = response {
        if let worker::Error::Json(json) = err {
            return Response::error(json.0, json.1).and_then(|response| {
                response
                    .with_headers(Headers::from_iter(
                        [("Content-Type", "application/json")].iter(),
                    ))
                    .with_cors(&CORS)
            });
        }

        return Response::error(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR.as_u16())
            .and_then(|response| {
                response
                    .with_headers(Headers::from_iter(
                        [("Content-Type", "application/json")].iter(),
                    ))
                    .with_cors(&CORS)
            });
    }

    response
}

fn root_handler() -> Response {
    Response::from_json(&gen_app_meta()).expect("from_json should've succeeded")
}
