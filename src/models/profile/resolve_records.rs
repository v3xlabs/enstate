use ethers::providers::Middleware;
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
        state: &AppState,
    ) -> Result<Option<String>, ProfileError> {
        let result = state
            .provider
            .resolve_field(name, record)
            .await
            .map_err(|e| {
                println!("Error resolving name: {e:?}");

                ProfileError::NotFound
            })?;

        Ok(result.is_empty().not().then_some(result))
    }

    pub async fn resolve_records(
        name: &str,
        state: &AppState,
    ) -> Result<BTreeMap<String, Option<String>>, ProfileError> {
        let mut results = BTreeMap::new();

        let mut futures = JoinSet::new();

        for record in &state.profile_records {
            let name = name.to_string();
            let state = state.clone();
            let record = record.clone();

            futures.spawn(async move {
                let result = Self::resolve_record(&name, &record, &state).await;

                (record, result)
            });
        }

        while let Some(res) = futures.join_next().await {
            let Ok((record, result)) = res else { continue };

            results.insert(record.to_string(), result.unwrap());
        }

        Ok(results)
    }
}

pub fn default_records() -> Vec<String> {
    vec![
        "url",
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
        "social.bsky",
        "org.telegram",
        "social.mastodon",
    ]
    .into_iter()
    .map(ToString::to_string)
    .collect()
}
