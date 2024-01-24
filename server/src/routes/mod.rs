use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::Json;
use enstate_shared::core::error::ProfileError;
use enstate_shared::utils::vec::dedup_ord;
use ethers::prelude::ProviderError;
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer};
use thiserror::Error;

use crate::models::error::ErrorResponse;

pub mod address;
pub mod four_oh_four;
pub mod header;
pub mod image;
pub mod name;
pub mod root;
pub mod universal;

// TODO (@antony1060): cleanup file

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
    Ok(value.map(|it| it == "true").unwrap_or(false))
}

pub type RouteError = (StatusCode, Json<ErrorResponse>);

impl From<ErrorResponse> for RouteError {
    fn from(value: ErrorResponse) -> Self {
        (
            StatusCode::from_u16(value.status).expect("status should be valid"),
            Json(value),
        )
    }
}

pub fn profile_http_error_mapper<T: AsRef<ProfileError>>(err: T) -> ErrorResponse {
    let err = err.as_ref();
    let status = match err {
        ProfileError::NotFound => StatusCode::NOT_FOUND,
        ProfileError::CCIPError(_) => StatusCode::BAD_GATEWAY,
        ProfileError::RPCError(ProviderError::EnsNotOwned(_)) => StatusCode::UNPROCESSABLE_ENTITY,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    ErrorResponse {
        status: status.as_u16(),
        error: err.to_string(),
    }
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

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("maximum input length exceeded (expected at most {0})")]
    MaxLengthExceeded(usize),
}

impl From<ValidationError> for RouteError {
    fn from(value: ValidationError) -> Self {
        http_error(StatusCode::BAD_REQUEST, &value.to_string())
    }
}

pub fn validate_bulk_input(
    input: &[String],
    max_len: usize,
) -> Result<Vec<String>, ValidationError> {
    let unique = dedup_ord(
        &input
            .iter()
            .map(|entry| entry.to_lowercase())
            .collect::<Vec<_>>(),
    );

    if unique.len() > max_len {
        return Err(ValidationError::MaxLengthExceeded(max_len));
    }

    Ok(unique)
}

pub struct Qs<T>(T);

lazy_static! {
    static ref SERDE_QS_CONFIG: serde_qs::Config = serde_qs::Config::new(2, false);
}

#[axum::async_trait]
impl<T, S> FromRequestParts<S> for Qs<T>
where
    T: serde::de::DeserializeOwned,
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or("");
        Ok(Self(
            SERDE_QS_CONFIG
                .deserialize_str(query)
                .map_err(|error| error.to_string())?,
        ))
    }
}
