use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use ethers::providers::Middleware;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NameResponse {
    pub address: String,
}

#[utoipa::path(
    get,
    path = "/n/{name}",
    responses(
        (status = 200, description = "Successfully found name.", body = NameResponse),
        (status = NOT_FOUND, description = "No name could be found."),
    ),
    params(
        ("name" = String, Path, description = "Name to lookup the address for."),
    )
)]
pub async fn get(
    Path(name): Path<String>,
    State(state): State<crate::AppState>,
) -> Result<Json<NameResponse>, StatusCode> {
    let v = state.provider.resolve_name(name.as_str());

    let result = v.await;

    let result = match result {
        Ok(result) => result,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    Ok(Json(NameResponse {
        address: format!("{:?}", result),
    }))
}
