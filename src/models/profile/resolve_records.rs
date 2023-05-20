use std::collections::HashMap;

use ethers::providers::Middleware;
use tokio::task::JoinSet;

use crate::{models::profile::Profile, state::AppState};

use super::ProfileError;

impl Profile {
    pub async fn resolve_record(
        name: &str,
        record: &str,
        state: &AppState,
    ) -> Result<String, ProfileError> {
        // Get the record of the name
        state
            .provider
            .resolve_field(name, record)
            .await
            .map_err(|e| {
                println!("Error resolving name: {e:?}");

                ProfileError::NotFound
            })
    }

    pub async fn resolve_records(
        name: &str,
        state: &AppState,
    ) -> Result<HashMap<String, String>, ProfileError> {
        let records = vec!["com.discord", "com.twitter", "display", "timezone"];
        let mut results = HashMap::new();

        let mut set = JoinSet::new();

        for record in records {
            let name = name.to_string();
            let state = state.clone();

            set.spawn(async move {
                let result = Profile::resolve_record(&name, record, &state).await;

                (record, result)
            });
        }

        while let Some(res) = set.join_next().await {
            let (record, result) = match res {
                Ok(res) => res,
                Err(_) => continue,
            };

            results.insert(record.to_string(), result.unwrap());
        }

        Ok(results)
    }
}
