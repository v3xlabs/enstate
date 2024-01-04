use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToSchema)]
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
