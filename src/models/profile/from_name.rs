use std::collections::BTreeMap;
use std::str::FromStr;

use ethers::providers::{Http, Provider};
use redis::AsyncCommands;
use tracing::info;

use crate::{
    models::{
        lookup::{addr::Addr, avatar::Avatar, ENSLookup, multicoin::Multicoin, text::Text},
        profile::Profile,
        universal_resolver::resolve_universal,
    },
    state::AppState,
};
use crate::utils::eip55::EIP55Address;

use super::error::ProfileError;

impl Profile {
    pub async fn from_name(
        name: &str,
        fresh: bool,
        state: &AppState,
    ) -> Result<Self, ProfileError> {
        let cache_key = format!("n:{name}");
        let mut redis = state.redis.clone();

        info!(
            name = name,
            cache_key = cache_key,
            fresh = fresh,
            "Looking up profile for {name}..."
        );

        // If the value is in the cache, return it
        if !fresh {
            if let Ok(value) = redis.get::<_, String>(&cache_key).await {
                if !value.is_empty() {
                    let entry: Self = serde_json::from_str(value.as_str()).unwrap();

                    return Ok(entry);
                }

                return Err(ProfileError::NotFound);
            }
        }

        let provider: Provider<Http> = state.provider.get_provider();

        // Preset Hardcoded Lookups
        let mut calldata: Vec<Box<dyn ENSLookup + Send + Sync>> = vec![
            Box::new(Addr {}),
            Box::new(Avatar {
                // TODO: Default IPFS Gateway
                ipfs_gateway: "https://ipfs.io/ipfs/".to_string(),
                name: name.to_string(),
            }),
            Box::new(Text::new("display".to_string())),
        ];

        // Lookup all Records
        let record_offset = calldata.len();
        for record in &state.profile_records {
            calldata.push(Box::new(Text::new(record.clone())));
        }

        // Lookup all chains
        let chain_offset = calldata.len();
        for chain in &state.profile_chains {
            calldata.push(Box::new(Multicoin {
                coin_type: chain.clone(),
            }));
        }

        // Execute Universal Resolver Lookup
        let (data, resolver) = resolve_universal(name.to_string(), &calldata, provider).await?;

        let mut results: Vec<Option<String>> = Vec::new();
        let mut errors = BTreeMap::default();

        // Assume results & calldata have the same length
        // Look through all calldata and decode the results at the same index
        for (index, calldata) in calldata.iter().enumerate() {
            let result = calldata.decode(&data[index]);

            match result {
                Ok(result) => {
                    results.push(Some(result));
                }
                Err(error) => {
                    errors.insert(calldata.name(), error.to_string());
                    results.push(None);
                }
            }
        }

        let address: Option<String> = results.get(0).unwrap_or(&None).clone();
        let avatar: Option<String> = results.get(1).unwrap_or(&None).clone();
        let display_record: Option<String> = results.get(3).unwrap_or(&None).clone();

        let display = match display_record {
            Some(display) if display.to_lowercase() == name.to_lowercase() => display,
            _ => name.to_string(),
        };

        info!(
            name = name,
            address,
            avatar = ?avatar,
            "Profile for {name} found"
        );

        let mut records = BTreeMap::default();

        for (index, value) in results[record_offset..chain_offset].iter().enumerate() {
            if let Some(value) = value {
                if !value.is_empty() {
                    records.insert(state.profile_records[index].clone(), value.to_string());
                }
            }
        }

        let mut chains = BTreeMap::default();

        for (index, value) in results[chain_offset..].iter().enumerate() {
            if let Some(value) = value {
                if !value.is_empty() {
                    chains.insert(state.profile_chains[index].to_string(), value.to_string());
                }
            }
        }

        let value = Self {
            name: name.to_string(),
            address: address.and_then(|it| EIP55Address::from_str(it.as_str()).ok()),
            avatar,
            display,
            records,
            chains,
            fresh: chrono::offset::Utc::now().timestamp_millis(),
            resolver: EIP55Address(resolver),
            errors,
        };

        let response = serde_json::to_string(&value).unwrap();

        redis
            .set_ex::<_, _, ()>(&cache_key, &response, 3600)
            .await
            .unwrap();

        Ok(value)
    }
}
