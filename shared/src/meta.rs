use lazy_static::lazy_static;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AppMeta {
    pub rev: String,
    pub name: String,
    pub version: String,
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
        .get(0)
        .map(|val| val.to_string())
        .unwrap_or_else(|| format!("git:{}", &commit_id));

    AppMeta {
        rev: commit_id.to_string(),
        version: tag,
        compile_time: build_info.timestamp.to_string(),
        name: "enstate".to_string(),
    }
}
