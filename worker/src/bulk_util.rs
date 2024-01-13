use crate::http_util::ValidationError;
use enstate_shared::utils::vec::dedup_ord;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BulkResponse<T> {
    pub(crate) response_length: usize,
    pub(crate) response: Vec<T>,
}

impl<T> From<Vec<T>> for BulkResponse<T> {
    fn from(value: Vec<T>) -> Self {
        BulkResponse {
            response_length: value.len(),
            response: value,
        }
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
