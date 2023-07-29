use ethers::providers::{Http, Middleware, Provider};
use ethers_ccip_read::CCIPReadMiddleware;
use std::{collections::BTreeMap, ops::Not};
use tokio::task::JoinSet;

use crate::{
    models::profile::{Profile, ProfileError},
    state::AppState,
};

impl Profile {
    pub async fn resolve_record(
        name: &str,
        record: &str,
        provider: CCIPReadMiddleware<Provider<Http>>,
    ) -> Result<Option<String>, ProfileError> {
        let result = provider
            .resolve_field(name, record)
            .await
            .map_err(|e| {
                println!("Error resolving name: {e:?}");

                ProfileError::NotFound
            })?;

        Ok(result.is_empty().not().then_some(result))
    }

    pub async fn resolve_records(name: &str, state: &AppState) -> BTreeMap<String, String> {
        let mut results: BTreeMap<String, String> = BTreeMap::new();

        let mut tasks: JoinSet<(String, Option<String>)> = JoinSet::new();

        for record in &state.profile_records {
            let name = name.to_string();
            let state = state.clone();
            let record = record.clone();

            let provider = state.get_random_rpc_provider().await;

            tasks.spawn(async move {
                let result = Self::resolve_record(&name, &record, provider)
                    .await
                    .ok()
                    .flatten();

                (record, result)
            });
        }

        while let Some(res) = tasks.join_next().await {
            let Ok((record, Some(result))) = res else { continue };

            results.insert(record.to_string(), result);
        }

        results
    }
}

pub fn default_records() -> Vec<String> {
    vec![
        "url",
        "name",
        "email",
        "header",
        "location",
        "timezone",
        "language",
        "pronouns",
        "com.github",
        "org.matrix",
        "io.keybase",
        "description",
        "com.twitter",
        "com.discord",
        "social.bsky",
        "org.telegram",
        "social.mastodon",
    ]
    .into_iter()
    .map(ToString::to_string)
    .collect()
}
