use enstate_shared::models::profile::ProfileService;
use http::StatusCode;
use worker::{Request, Response, RouteContext};

use crate::http_util::{
    http_simple_status_error, parse_query, profile_http_error_mapper, redirect_url, FreshQuery,
};

pub async fn get(req: Request, ctx: RouteContext<ProfileService>) -> worker::Result<Response> {
    let query: FreshQuery = parse_query(&req)?;

    let name_or_address = ctx
        .param("name_or_address")
        .ok_or_else(|| http_simple_status_error(StatusCode::BAD_REQUEST))?;

    let profile = ctx
        .data
        .resolve_from_name_or_address(name_or_address, query.fresh)
        .await
        .map_err(profile_http_error_mapper)?;

    let Some(avatar) = profile.avatar else {
        return Err(http_simple_status_error(StatusCode::NOT_FOUND));
    };

    redirect_url(&avatar)
}
