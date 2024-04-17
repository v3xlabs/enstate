use crate::models::bulk::{BulkResponse, ListResponse};
use crate::models::error::ErrorResponse;
use crate::models::profile::ENSProfile;
use utoipa::openapi::{ExternalDocs, License};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "enstate.rs",
        description = "A hosted ENS API allowing for easy access to ENS data.",
    ),
    paths(crate::routes::address::get, crate::routes::name::get, crate::routes::universal::get),
    components(schemas(ENSProfile, ListResponse<BulkResponse<ENSProfile>>, ErrorResponse))
)]
pub struct ApiDoc;

pub async fn openapi() -> String {
    let mut doc = ApiDoc::openapi();

    let license = License::new("GPLv3");

    doc.info.license = Some(license);
    doc.external_docs = Some(ExternalDocs::new("https://github.com/v3xlabs/enstate"));

    doc.to_json().unwrap()
}
