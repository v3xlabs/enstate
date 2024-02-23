use enstate_shared::core::error::ProfileError;
use enstate_shared::core::lookup_data::LookupInfo;
use enstate_shared::core::ENSService;
use enstate_shared::models::lookup::ENSLookup;
use http::StatusCode;
use worker::{Headers, Request, Response, RouteContext};

use crate::http_util::{
    http_simple_status_error, parse_query, profile_http_error_mapper, redirect_url, FreshQuery,
};

pub async fn get(req: Request, ctx: RouteContext<ENSService>) -> worker::Result<Response> {
    let query: FreshQuery = parse_query(&req)?;

    let name_or_address = ctx
        .param("name_or_address")
        .ok_or_else(|| http_simple_status_error(StatusCode::BAD_REQUEST))?;

    let info = LookupInfo::guess(name_or_address)
        .map_err(|_| profile_http_error_mapper(ProfileError::NotFound))?;

    let avatar = ctx
        .data
        .resolve_record_simple(info, ENSLookup::Image("avatar".to_string()), query.fresh)
        .await
        .map_err(profile_http_error_mapper)?;

    if let Some(processed) = enstate_shared::utils::data_url::process_data_url_image(&avatar) {
        let Ok(processed) = processed else {
            return Err(http_simple_status_error(StatusCode::UNSUPPORTED_MEDIA_TYPE));
        };

        return Ok(Response::from_bytes(processed.data)
            .map_err(|_| http_simple_status_error(StatusCode::INTERNAL_SERVER_ERROR))?
            .with_headers(Headers::from_iter([(
                "Content-Type",
                processed.mimetype.as_str(),
            )])));
    }

    redirect_url(&avatar)
}
