use crate::models::error::ErrorResponse;
use axum::http::StatusCode;
use axum::Json;

use crate::routes::RouteError;

pub async fn handler() -> RouteError {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            status: StatusCode::NOT_FOUND.as_u16(),
            error: "Unknown route".to_string(),
        }),
    )
}
