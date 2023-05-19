use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::models::profile_data::ProfileData;

#[utoipa::path(
    get,
    path = "/a/{address}",
    responses(
        (status = 200, description = "Successfully found address", body = ProfileData),
        (status = BAD_REQUEST, description = "Invalid address."),
        (status = NOT_FOUND, description = "No name was associated with this address."),
        (status = UNPROCESSABLE_ENTITY, description = "Reverse record not owned by this address."),
    ),
    params(
        ("address" = String, Path, description = "Address to lookup name data for"),
    )
)]
pub async fn get(
    Path(address): Path<String>,
    State(state): State<crate::AppState>,
) -> Result<Json<ProfileData>, StatusCode> {
    let address = address.parse().map_err(|_| StatusCode::BAD_REQUEST)?;

    (ProfileData::from_address(address, &state).await)
        .map_or(Err(StatusCode::NOT_FOUND), |profile_data| {
            Ok(Json(profile_data))
        })
}
