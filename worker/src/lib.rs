use enstate_shared::meta::gen_app_meta;
use lazy_static::lazy_static;
use worker::{event, Context, Cors, Env, Method, Request, Response, RouteContext, Router};

use crate::lookup::LookupType;

mod http_util;
mod kv_cache;
mod lookup;

lazy_static! {
    static ref CORS: Cors = Cors::default()
        .with_origins(vec!["*"])
        .with_methods(Method::all());
}

async fn main_handler(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let opensea_api_key = ctx
        .env
        .var("OPENSEA_API_KEY")
        .expect("OPENSEA_API_KEY should've been set")
        .to_string();

    LookupType::from(req.path())
        .process(req, ctx.env, &opensea_api_key)
        .await
        .unwrap_or_else(|f| f)
        .with_cors(&CORS)
}

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    Router::new()
        .get("/", |_, _| root_handler().with_cors(&CORS))
        .get_async("/:method/:param", main_handler)
        .run(req, env)
        .await
}

fn root_handler() -> Response {
    Response::from_json(&gen_app_meta()).expect("from_json should've succeeded")
}
