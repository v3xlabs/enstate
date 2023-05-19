use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RecordsResponse {
    pub address: String,
}

#[utoipa::path(
    get,
    path = "/r/{name}",
    responses(
        (status = 200, description = "Successfully found name.", body = RecordsResponse),
        (status = NOT_FOUND, description = "No name could be found."),
    ),
    params(
        ("name" = String, Path, description = "Name to lookup the address for."),
    )
)]
pub async fn get() -> Json<RecordsResponse> {
    Json(RecordsResponse {
        address: "0x1234567890".to_string(),
    })
}
