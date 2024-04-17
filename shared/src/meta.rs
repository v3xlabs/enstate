use lazy_static::lazy_static;
use utoipa::ToSchema;

#[derive(Debug, Clone, serde::Serialize, ToSchema)]
pub struct AppMeta {
    #[schema(example = "fcf86f91")]
    pub rev: String,
    #[schema(example = "enstate")]
    pub name: String,
    #[schema(example = "git:fcf86f91")]
    pub version: String,
    #[schema(example = "2024-04-17 16:27:49.963738487 UTC")]
    pub compile_time: String,
}

build_info::build_info!(fn build_info);

lazy_static! {
    pub static ref APP_META: AppMeta = gen_app_meta();
}

pub fn gen_app_meta() -> AppMeta {
    let build_info = build_info();
    let vc_info = build_info.version_control.as_ref().unwrap();
    let info = vc_info.git().unwrap();

    let commit_id: String = info.commit_id.chars().take(8).collect();

    let tag = info
        .tags
        .first()
        .map(|val| val.to_string())
        .unwrap_or_else(|| format!("git:{}", &commit_id));

    AppMeta {
        rev: commit_id.to_string(),
        version: tag,
        compile_time: build_info.timestamp.to_string(),
        name: "enstate".to_string(),
    }
}
