use lazy_static::lazy_static;
use std::sync::Arc;

use worker::{event, Context, Cors, Env, Method, Request, Response};

use crate::lookup::LookupType;

mod http_util;
mod kv_cache;
mod lookup;

lazy_static! {
    static ref CORS: Cors = Cors::default()
        .with_origins(vec!["*"])
        .with_methods(Method::all());
}

#[derive(Debug, serde::Serialize)]
struct AppVersion {
    rev: String,
    name: String,
    semver: String,
    compile_time: String,
}

#[event(fetch, respond_with_errors)]
async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    if req.path() == "/" {
        return root_handler().with_cors(&CORS);
    }

    let opensea_api_key = env
        .var("OPENSEA_API_KEY")
        .expect("OPENSEA_API_KEY should've been set")
        .to_string();

    let env_arc = Arc::new(env);

    let response = LookupType::from(req.path())
        .process(req, env_arc, &opensea_api_key)
        .await
        .unwrap_or_else(|f| f);

    response.with_cors(&CORS)
}

fn root_handler() -> Response {
    Response::from_json(&AppVersion {
        rev: env!("GIT_REV").to_string(),
        name: env!("CARGO_PKG_NAME").to_string(),
        semver: env!("CARGO_PKG_VERSION").to_string(),
        compile_time: env!("STATIC_BUILD_DATE").to_string(),
    })
    .expect("from_json should've succeeded")
}
