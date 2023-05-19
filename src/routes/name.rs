use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::models::profile::Profile;

#[utoipa::path(
    get,
    path = "/n/{name}",
    responses(
        (status = 200, description = "Successfully found name.", body = ProfileData),
        (status = NOT_FOUND, description = "No name could be found."),
    ),
    params(
        ("name" = String, Path, description = "Name to lookup the address for."),
    )
)]
pub async fn get(
    Path(name): Path<String>,
    State(state): State<crate::AppState>,
) -> Result<Json<Profile>, StatusCode> {
    let profile = Profile::from_name(&name, &state)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(profile))
}
