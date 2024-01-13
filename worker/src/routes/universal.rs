use enstate_shared::models::profile::{Profile, ProfileService};
use futures_util::future::join_all;
use http::StatusCode;
use serde::Deserialize;
use worker::{Request, Response, RouteContext};

use crate::bulk_util::{validate_bulk_input, BulkResponse, ListResponse};
use crate::http_util::{
    http_simple_status_error, parse_query, profile_http_error_mapper, FreshQuery,
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

    Response::from_json(&profile)
}

#[derive(Deserialize)]
pub struct UniversalGetBulkQuery {
    queries: Vec<String>,

    #[serde(flatten)]
    fresh: FreshQuery,
}

pub async fn get_bulk(req: Request, ctx: RouteContext<ProfileService>) -> worker::Result<Response> {
    let query: UniversalGetBulkQuery = parse_query(&req)?;

    let queries = validate_bulk_input(&query.queries, 10)?;

    let profiles = queries
        .iter()
        .map(|input| {
            ctx.data
                .resolve_from_name_or_address(input, query.fresh.fresh)
        })
        .collect::<Vec<_>>();

    let joined: ListResponse<BulkResponse<Profile>> = join_all(profiles).await.into();

    Response::from_json(&joined)
}
