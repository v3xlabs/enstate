use enstate_shared::models::profile::error::ProfileError;
use enstate_shared::utils::vec::dedup_ord;
use ethers::prelude::ProviderError;
use http::status::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};
use thiserror::Error;
use worker::{Error, Request, Response, Url};

// TODO (@antony1060): cleanup file

#[derive(Deserialize)]
pub struct FreshQuery {
    #[serde(default, deserialize_with = "bool_or_false")]
    pub(crate) fresh: bool,
}

#[allow(clippy::unnecessary_wraps)]
fn bool_or_false<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Result<String, D::Error> = Deserialize::deserialize(deserializer);
    Ok(value.map(|it| it == "true").unwrap_or(false))
}

pub fn parse_query<T: DeserializeOwned>(req: &Request) -> worker::Result<T> {
    let url = req.url()?;
    let query = url.query().unwrap_or("");

    serde_qs::from_str::<T>(query).map_err(|_| http_simple_status_error(StatusCode::BAD_REQUEST))
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("maximum input length exceeded (expected at most {0})")]
    MaxLengthExceeded(usize),
}

impl From<ValidationError> for worker::Error {
    fn from(value: ValidationError) -> Self {
        ErrorResponse {
            status: StatusCode::BAD_REQUEST.as_u16(),
            error: value.to_string(),
        }
        .into()
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

#[derive(Serialize)]
pub struct ErrorResponse {
    pub(crate) status: u16,
    pub(crate) error: String,
}

impl From<ErrorResponse> for Error {
    fn from(value: ErrorResponse) -> Self {
        let json = serde_json::to_string(&value).expect("error should be json serializable");

        Self::Json((json, value.status))
    }
}

pub fn profile_http_error_mapper(err: ProfileError) -> Error {
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
    .into()
}

pub fn http_simple_status_error(status: StatusCode) -> Error {
    ErrorResponse {
        status: status.as_u16(),
        error: status
            .canonical_reason()
            .unwrap_or("Unknown error")
            .to_string(),
    }
    .into()
}

pub fn redirect_url(url: &str) -> worker::Result<Response> {
    let url = Url::parse(url).map_err(|_| {
        worker::Error::from(ErrorResponse {
            status: StatusCode::NOT_ACCEPTABLE.as_u16(),
            error: "invalid avatar url".to_string(),
        })
    })?;

    Response::redirect(url)
}
