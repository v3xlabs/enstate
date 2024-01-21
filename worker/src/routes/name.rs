use enstate_shared::core::lookup_data::LookupInfo;
use enstate_shared::core::{ENSService, Profile};
use futures_util::future::join_all;
use http::StatusCode;
use serde::Deserialize;
use worker::{Request, Response, RouteContext};

use crate::bulk_util::{validate_bulk_input, BulkResponse, ListResponse};
use crate::http_util::{
    http_simple_status_error, parse_query, profile_http_error_mapper, FreshQuery,
};

pub async fn get(req: Request, ctx: RouteContext<ENSService>) -> worker::Result<Response> {
    let query: FreshQuery = parse_query(&req)?;

    let name = ctx
        .param("name")
        .ok_or_else(|| http_simple_status_error(StatusCode::BAD_REQUEST))?;

    let profile = ctx
        .data
        .resolve_profile(LookupInfo::Name(name.to_string()), query.fresh)
        .await
        .map_err(profile_http_error_mapper)?;

    Response::from_json(&profile)
}

#[derive(Deserialize)]
pub struct NameGetBulkQuery {
    names: Vec<String>,

    #[serde(flatten)]
    fresh: FreshQuery,
}

pub async fn get_bulk(req: Request, ctx: RouteContext<ENSService>) -> worker::Result<Response> {
    let query: NameGetBulkQuery = parse_query(&req)?;

    let names = validate_bulk_input(&query.names, 10)?;

    let profiles = names
        .iter()
        .map(|name| {
            ctx.data
                .resolve_profile(LookupInfo::Name(name.to_string()), query.fresh.fresh)
        })
        .collect::<Vec<_>>();

    let joined: ListResponse<BulkResponse<Profile>> = join_all(profiles).await.into();

    Response::from_json(&joined)
}
