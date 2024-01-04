use enstate_shared::models::profile::ProfileService;
use ethers::addressbook::Address;
use futures_util::future::try_join_all;
use http::StatusCode;
use serde::Deserialize;
use worker::{Request, Response, RouteContext};

use crate::http_util::{
    http_simple_status_error, parse_query, profile_http_error_mapper, validate_bulk_input,
    FreshQuery,
};

pub async fn get(req: Request, ctx: RouteContext<ProfileService>) -> worker::Result<Response> {
    let query: FreshQuery = parse_query(&req)?;

    let address = ctx
        .param("address")
        .ok_or_else(|| http_simple_status_error(StatusCode::BAD_REQUEST))?;

    let address: Address = address
        .parse()
        .map_err(|_| http_simple_status_error(StatusCode::BAD_REQUEST))?;

    let profile = ctx
        .data
        .resolve_from_address(address, query.fresh)
        .await
        .map_err(profile_http_error_mapper)?;

    Response::from_json(&profile)
}

#[derive(Deserialize)]
pub struct AddressGetBulkQuery {
    addresses: Vec<String>,

    #[serde(flatten)]
    fresh: FreshQuery,
}

pub async fn get_bulk(req: Request, ctx: RouteContext<ProfileService>) -> worker::Result<Response> {
    let query: AddressGetBulkQuery = parse_query(&req)?;

    let addresses = validate_bulk_input(&query.addresses, 10)?;

    let addresses = addresses
        .iter()
        .map(|address| address.parse::<Address>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| http_simple_status_error(StatusCode::BAD_REQUEST))?;

    let profiles = addresses
        .iter()
        .map(|address| ctx.data.resolve_from_address(*address, query.fresh.fresh))
        .collect::<Vec<_>>();

    let joined = try_join_all(profiles)
        .await
        .map_err(profile_http_error_mapper)?;

    Response::from_json(&joined)
}
