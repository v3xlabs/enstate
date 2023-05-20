use crate::{models::profile::Profile, state::AppState};

impl Profile {
    pub async fn resolve_display(name: &str, state: &AppState) -> Option<String> {
        let Ok(Some(display)) = Self::resolve_record(name, "display", state).await else { return None };

        (name.to_lowercase() == display.to_lowercase()).then_some(display)
    }
}
