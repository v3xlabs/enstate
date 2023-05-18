use crate::routes;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(routes::address::get, routes::name::get),
    components(schemas(routes::address::AddressResponse, routes::name::NameResponse))
)]
pub struct ApiDoc;
