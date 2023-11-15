use std::sync::Arc;

use worker::{event, Context, Cors, Env, Method, Request, Response};

use crate::lookup::LookupType;

mod http_util;
mod kv_cache;
mod lookup;

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    let cors = Cors::default()
        .with_origins(vec!["*"])
        .with_methods(Method::all());

    let opensea_api_key = env
        .var("OPENSEA_API_KEY")
        .expect("OPENSEA_API_KEY should've been set")
        .to_string();

    let env_arc = Arc::new(env);

    let response = LookupType::from(req.path())
        .process(req, env_arc, &opensea_api_key)
        .await
        .unwrap_or_else(|f| f);

    response.with_cors(&cors)
}
