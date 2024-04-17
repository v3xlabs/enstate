use crate::models::bulk::{BulkResponse, ListResponse};
use crate::models::error::ErrorResponse;
use crate::models::profile::ENSProfile;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    // paths(crate::routes::address::get, crate::routes::name::get, crate::routes::universal::get),
    components(schemas(ENSProfile, ListResponse<BulkResponse<ENSProfile>>, ErrorResponse))
)]
pub struct ApiDoc;

pub async fn openapi() -> String {
    ApiDoc::openapi().to_json().unwrap()
}
