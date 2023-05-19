use crate::routes;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(routes::address::get, routes::name::get),
    components(schemas(crate::models::profile_data::ProfileData))
)]
pub struct ApiDoc;
