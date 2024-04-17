use utoipa::ToSchema;

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub(crate) status: u16,
    pub(crate) error: String,
}
