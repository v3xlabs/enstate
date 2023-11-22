use enstate_shared::models::profile::error::ProfileError;
use http::status::StatusCode;
use serde::Serialize;
use worker::Response;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub(crate) status: u16,
    pub(crate) error: String,
}

pub fn profile_http_error_mapper(err: ProfileError) -> Response {
    let status = match err {
        ProfileError::NotFound => StatusCode::NOT_FOUND,
        ProfileError::CCIPError(_) => StatusCode::BAD_GATEWAY,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    Response::from_json(&ErrorResponse {
        status: status.as_u16(),
        error: err.to_string(),
    })
    .expect("from_json should've succeeded")
    .with_status(status.as_u16())
}

pub fn http_simple_status_error(status: StatusCode) -> Response {
    Response::from_json(&ErrorResponse {
        status: status.as_u16(),
        error: status
            .canonical_reason()
            .unwrap_or("Unknown error")
            .to_string(),
    })
    .expect("from_json shoud've succeeded")
    .with_status(status.as_u16())
}
