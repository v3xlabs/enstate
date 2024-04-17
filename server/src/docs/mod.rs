use crate::models::bulk::{BulkResponse, ListResponse};
use crate::models::error::ErrorResponse;
use crate::models::profile::ENSProfile;
use enstate_shared::meta::AppMeta;
use enstate_shared::utils::vec;
use utoipa::openapi::{ExternalDocs, License, Tag};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "enstate.rs",
        description = "A hosted ENS API allowing for easy access to ENS data.",
    ),
    paths(
        crate::routes::address::get, crate::routes::name::get, crate::routes::universal::get,
        crate::routes::address::get_bulk, crate::routes::name::get_bulk, crate::routes::universal::get_bulk,
        crate::routes::address::get_bulk_sse, crate::routes::name::get_bulk_sse, crate::routes::universal::get_bulk_sse,
        crate::routes::header::get,
        crate::routes::image::get,
        crate::routes::root::get,
    ),
    components(schemas(ENSProfile, ListResponse<BulkResponse<ENSProfile>>, ErrorResponse, AppMeta))
)]
pub struct ApiDoc;

pub async fn openapi() -> String {
    let mut doc = ApiDoc::openapi();

    let license = License::new("GPLv3");

    doc.info.license = Some(license);
    doc.external_docs = Some(ExternalDocs::new("https://github.com/v3xlabs/enstate"));

    let mut tag1 = Tag::default();
    tag1.name = "Single Profile".to_string();
    tag1.description = Some("If you want to resolve a single ENS Name / Profile you can do a single lookup in one of the following ways.".to_string());
    let mut tag2 = Tag::default();
    tag2.name = "Bulk Profiles".to_string();
    tag2.description = Some("In some cases you might want to resolve a list of names or addresses. In this case you can use the bulk endpoints. This endpoint waits for all names to be resolved before returning a result.\n\nNote: You might prefer to use the [SSE Streaming Endpoints](#tag/stream-profiles) for a more responsive feel.".to_string());
    let mut tag3 = Tag::default();
    tag3.name = "Stream Profiles".to_string();
    tag3.description = Some("In some cases you might want to resolve a list of names or addresses but have access to the results immediately. This endpoint returns its output via [Server Sent Events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events) as they are computed.\n\nNote: If you are looking for a simpler solution that return result aggregates checkout [Bulk Endpoints](#tag/bulk-profiles).".to_string());
    let mut tag4 = Tag::default();
    tag4.name = "Avatars & Banners".to_string();
    tag4.description = Some("To save you the hassle of loading profiles, and extracting json fields, we have made a few endpoints that will make it easy for you to directly use image urls in your app.".to_string());
    let mut tag5 = Tag::default();
    tag5.name = "Deployment Information".to_string();

    doc.tags = Some(vec![tag1, tag2, tag3, tag4, tag5]);

    doc.to_json().unwrap()
}
