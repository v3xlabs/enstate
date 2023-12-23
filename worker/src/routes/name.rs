use enstate_shared::models::profile::ProfileService;
use http::StatusCode;
use worker::{Request, Response, RouteContext};

use crate::http_util::{
    http_simple_status_error, parse_query, profile_http_error_mapper, FreshQuery,
};

pub async fn get(req: Request, ctx: RouteContext<ProfileService>) -> worker::Result<Response> {
    let query: FreshQuery = parse_query(&req)?;

    let name = ctx
        .param("name")
        .ok_or_else(|| http_simple_status_error(StatusCode::BAD_REQUEST))?;

    let profile = ctx
        .data
        .resolve_from_name(name, query.fresh)
        .await
        .map_err(profile_http_error_mapper)?;

    Response::from_json(&profile)
}
