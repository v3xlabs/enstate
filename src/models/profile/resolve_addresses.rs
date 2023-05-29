use ethers::providers::Middleware;
use std::{collections::BTreeMap, ops::Not};
use tokio::task::JoinSet;

use crate::{
    models::profile::{Profile, ProfileError},
    state::AppState,
};

// 11101101
// 00011100
// 11101110
// 00001111
// 11100001
// 11100111
// 10111100
// 11110010
// 11001010
// 00110011
// 11000010
// 10001010
// 10000000
// 01101110
// 11100010
// 01100101
// 11001111
// 11101101
// 11110000
// 00101111
// 11101101
// 11110001
// 10110001
// 00100100
// 11001010
// 01110011
// 10110010
// 00100000
// 00111100
// 10101000
// 00001100
// 11000111
// 11001001
// 00011010
// 00000010
// 10101101
// 00111100

// The above binary but every character as hex and all in one line
// / ed1c ee0f / e17b f2ca 33c2 8a80 6ee2 65cf edf0 2fed b124 73b2 20ac 0c87 c92a 02ad / 3c

impl Profile {
    /**
         * 0xed1cee0fe1e7bcf2ca33c28a806ee265cfedf02fedf1b124ca73b2203ca80cc7c91a02ad000000000000000000000000000000000000000000000000000000000000003c
    0xf1cb7e06e1e7bcf2ca33c28a806ee265cfedf02fedf1b124ca73b2203ca80cc7c91a02ad000000000000000000000000000000000000000000000000000000000000003c
    11110001 241
    11001011 203
    01111110 126
    00000110 6
    [241, 203, 126, 6]
         */
    pub async fn resolve_address(
        name: &str,
        coin_type: u8,
        state: &AppState,
    ) -> Result<Option<String>, ProfileError> {
        Err(ProfileError::NotFound)

        // let result = state
        //     .provider
        //     .resolve_addresses(name, "60")
        //     .await
        //     .map_err(|e| {
        //         println!("Error resolving name: {e:?}");

        //         ProfileError::NotFound
        //     })?;

        // Ok(result.is_empty().not().then_some(result))
    }

    pub async fn resolve_addresses(name: &str, state: &AppState) -> BTreeMap<String, String> {
        let mut results: BTreeMap<String, String> = BTreeMap::new();

        // let res = Self::resolve_address(name, 60, state).await.ok().flatten();

        // results.insert("ETH".to_owned(), res.unwrap_or_default());

        // let mut tasks: JoinSet<(String, Option<String>)> = JoinSet::new();

        // for record in &state.profile_records {
        //     let name = name.to_string();
        //     let state = state.clone();
        //     let record = record.clone();

        //     tasks.spawn(async move {
        //         let result = Self::resolve_record(&name, &record, &state)
        //             .await
        //             .ok()
        //             .flatten();

        //         (record, result)
        //     });
        // }

        // while let Some(res) = tasks.join_next().await {
        //     let Ok((record, Some(result))) = res else { continue };

        //     results.insert(record.to_string(), result);
        // }

        results
    }
}

pub fn default_addresses() -> Vec<String> {
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
        "social.bsky",
        "org.telegram",
        "social.mastodon",
    ]
    .into_iter()
    .map(ToString::to_string)
    .collect()
}
