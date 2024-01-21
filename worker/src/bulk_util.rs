use crate::http_util::{profile_http_error_mapper, ErrorResponse, ValidationError};
use enstate_shared::core::error::ProfileError;
use enstate_shared::core::Profile;
use enstate_shared::utils::vec::dedup_ord;

#[derive(serde::Serialize)]
#[serde(tag = "type")]
pub enum BulkResponse<Ok> {
    #[serde(rename = "success")]
    Ok(Ok),
    #[serde(rename = "error")]
    Err(ErrorResponse),
}

impl<T> From<BulkResponse<T>> for Result<T, ErrorResponse> {
    fn from(value: BulkResponse<T>) -> Self {
        match value {
            BulkResponse::Ok(value) => Ok(value),
            BulkResponse::Err(err) => Err(err),
        }
    }
}

impl<T> From<Result<T, ErrorResponse>> for BulkResponse<T> {
    fn from(value: Result<T, ErrorResponse>) -> Self {
        match value {
            Ok(value) => Self::Ok(value),
            Err(err) => Self::Err(err),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct ListResponse<T> {
    pub(crate) response_length: usize,
    pub(crate) response: Vec<T>,
}

impl<T> From<Vec<T>> for ListResponse<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            response_length: value.len(),
            response: value,
        }
    }
}

impl From<Vec<Result<Profile, ProfileError>>> for ListResponse<BulkResponse<Profile>> {
    fn from(value: Vec<Result<Profile, ProfileError>>) -> Self {
        value
            .into_iter()
            .map(|it| it.map_err(profile_http_error_mapper))
            .map(BulkResponse::from)
            .collect::<Vec<_>>()
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
