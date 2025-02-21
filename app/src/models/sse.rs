use enstate_shared::core::Profile;

use crate::models::bulk::BulkResponse;

#[derive(Debug, serde::Serialize)]
pub struct SSEResponse {
    pub(crate) query: String,
    pub(crate) response: BulkResponse<Profile>,
}
