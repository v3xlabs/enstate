use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::Json;
use enstate_shared::models::profile::error::ProfileError;
use enstate_shared::models::profile::Profile;
use enstate_shared::utils::vec::dedup_ord;
use ethers::prelude::ProviderError;
use ethers::providers::{Http, Provider};
use ethers_core::types::Address;
use serde::{Deserialize, Deserializer};

use crate::models::error::ErrorResponse;

pub mod address;
pub mod four_oh_four;
pub mod header;
pub mod image;
pub mod name;
pub mod root;
pub mod universal;

#[derive(Deserialize)]
pub struct FreshQuery {
    #[serde(default, deserialize_with = "bool_or_false")]
    fresh: bool,
}

#[allow(clippy::unnecessary_wraps)]
fn bool_or_false<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Result<String, D::Error> = Deserialize::deserialize(deserializer);
    // FIXME (@antony1060):
    Ok(value.map(|it| it == "true").unwrap_or(false))
}

pub type RouteError = (StatusCode, Json<ErrorResponse>);

pub fn profile_http_error_mapper<T: AsRef<ProfileError>>(err: T) -> RouteError {
    let err = err.as_ref();
    let status = match err {
        ProfileError::NotFound => StatusCode::NOT_FOUND,
        ProfileError::CCIPError(_) => StatusCode::BAD_GATEWAY,
        ProfileError::RPCError(ProviderError::EnsNotOwned(_)) => StatusCode::UNPROCESSABLE_ENTITY,
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

pub fn http_error(status: StatusCode, error: &str) -> RouteError {
    (
        status,
        Json(ErrorResponse {
            status: status.as_u16(),
            error: error.to_string(),
        }),
    )
}

pub async fn universal_profile_resolve(
    name_or_address: &str,
    fresh: bool,
    rpc: Provider<Http>,
    state: &crate::AppState,
) -> Result<Profile, ProfileError> {
    let opensea_api_key = &state.opensea_api_key;

    let cache = state.cache.as_ref().as_ref();

    if let Ok(address) = name_or_address.parse::<Address>() {
        return Profile::from_address(
            address,
            fresh,
            cache,
            rpc,
            opensea_api_key,
            &state.profile_records,
            &state.profile_chains,
        )
        .await;
    }

    if !enstate_shared::patterns::test_domain(name_or_address) {
        return Err(ProfileError::NotFound);
    }

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

// TODO (@antony1060): None only happens when input length > max_len
//  result is more appropriate
pub fn validate_bulk_input(input: &[String], max_len: usize) -> Option<Vec<String>> {
    let unique = dedup_ord(input);

    if unique.len() > max_len {
        return None;
    }

    Some(unique)
}

pub struct Qs<T>(T);

#[axum::async_trait]
impl<T, S> FromRequestParts<S> for Qs<T>
where
    T: serde::de::DeserializeOwned,
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or("");
        Ok(Self(
            serde_qs::from_str(query).map_err(|error| error.to_string())?,
        ))
    }
}
