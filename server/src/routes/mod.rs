use axum::http::StatusCode;
use axum::Json;
use enstate_shared::models::profile::error::ProfileError;
use enstate_shared::models::profile::Profile;
use ethers::providers::{Http, Provider};
use ethers_core::types::Address;
use serde::Deserialize;

use crate::cache;
use crate::models::error::ErrorResponse;

pub mod address;
pub mod image;
pub mod name;
pub mod root;
pub mod universal;

pub mod four_oh_four;

#[derive(Deserialize)]
pub struct FreshQuery {
    fresh: Option<bool>,
}

pub type RouteError = (StatusCode, Json<ErrorResponse>);

pub fn profile_http_error_mapper(err: ProfileError) -> RouteError {
    let status = match err {
        ProfileError::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    (
        status,
        Json(ErrorResponse {
            status: status.as_u16(),
            error: err.to_string(),
        }),
    )
}

pub fn http_simple_status_error(status: StatusCode) -> RouteError {
    (
        status,
        Json(ErrorResponse {
            status: status.as_u16(),
            error: status
                .canonical_reason()
                .unwrap_or("Unknown error")
                .to_string(),
        }),
    )
}

pub async fn universal_profile_resolve(
    name_or_address: &str,
    fresh: bool,
    rpc: Provider<Http>,
    state: &crate::AppState,
) -> Result<Profile, ProfileError> {
    let cache = Box::new(cache::Redis::new(state.redis.clone()));

    let opensea_api_key = &state.opensea_api_key;

    let address_option: Option<Address> = name_or_address.parse().ok();

    match address_option {
        Some(address) => {
            Profile::from_address(
                address,
                fresh,
                cache,
                rpc,
                opensea_api_key,
                &state.profile_records,
                &state.profile_chains,
            )
            .await
        }
        None => {
            Profile::from_name(
                &name_or_address.to_lowercase(),
                fresh,
                cache,
                rpc,
                opensea_api_key,
                &state.profile_records,
                &state.profile_chains,
            )
            .await
        }
    }
}
