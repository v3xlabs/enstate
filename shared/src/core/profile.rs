use std::collections::{BTreeMap, HashSet};
use std::str::FromStr;

use ethers::prelude::{Middleware, MiddlewareBuilder};
use ethers_ccip_read::CCIPReadMiddleware;
use tracing::{info, instrument};

use crate::cache::CacheError;
use crate::core::{ENSService, Profile};
use crate::core::error::ProfileError;
use crate::core::lookup_data::LookupInfo;
use crate::models::lookup::ENSLookup;
use crate::utils::eip55::EIP55Address;

impl ENSService {
    #[instrument(skip(self))]
    pub async fn resolve_profile(
        &self,
        lookup: LookupInfo,
        fresh: bool,
    ) -> Result<Profile, ProfileError> {
        let name = match lookup {
            LookupInfo::Name(name) => name,
            LookupInfo::Address(address) => self
                .primary_from_address(&address, fresh)
                .await
                .map_err(|_| ProfileError::NotFound)?,
        };

        let cache_key = format!("n:{name}");

        let rpc = self.rpc.get_instance();

        let rpc = rpc.wrap_into(CCIPReadMiddleware::new);

        info!(
            name = name,
            cache_key = cache_key,
            fresh = fresh,
            rpc_url = rpc.inner().url().to_string(),
            "Looking up profile for {name}..."
        );

        // If the value is in the cache, return it
        if !fresh {
            if let Ok(value) = self.cache.get(&cache_key).await {
                if value.is_empty() {
                    return Err(ProfileError::NotFound);
                }

                let entry_result: Result<Profile, _> = serde_json::from_str(value.as_str());
                if let Ok(entry) = entry_result {
                    return Ok(entry);
                }
            }
        }

        // Preset Hardcoded Lookups
        let mut calldata: HashSet<ENSLookup> = HashSet::new();

        let (addr_key, avatar_key, header_key, display_key) = (
            ENSLookup::Addr,
            ENSLookup::StaticImage("avatar"),
            ENSLookup::StaticImage("header"),
            ENSLookup::StaticText("display"),
        );

        calldata.extend([
            addr_key.clone(),
            avatar_key.clone(),
            header_key.clone(),
            display_key.clone(),
        ]);

        calldata.extend(self.profile_records.iter().cloned().map(ENSLookup::Text));
        calldata.extend(
            self.profile_chains
                .iter()
                .cloned()
                .map(ENSLookup::Multicoin),
        );

        let resolved = self
            .resolve_records(
                LookupInfo::Name(name.to_string()),
                &Vec::from_iter(calldata),
                fresh,
            )
            .await?;

        let address = resolved.records.get(&addr_key).cloned();
        let avatar = resolved.records.get(&avatar_key).cloned();
        let header = resolved.records.get(&header_key).cloned();
        let display_record = resolved.records.get(&display_key).cloned();

        let display = match display_record {
            Some(display) if display.to_lowercase() == name.to_lowercase() => display,
            _ => name.to_string(),
        };

        info!(
            name = name,
            address,
            avatar = ?avatar,
            header = ?header,
            "Profile for {name} found"
        );

        let records: BTreeMap<String, String> = self
            .profile_records
            .iter()
            .filter_map(|record| {
                resolved
                    .records
                    .get(&ENSLookup::Text(record.to_string()))
                    .map(|value| (record.to_string(), value.to_string()))
            })
            .collect();

        let chains: BTreeMap<String, String> = self
            .profile_chains
            .iter()
            .filter_map(|coin_type| {
                resolved
                    .records
                    .get(&ENSLookup::Multicoin(coin_type.clone()))
                    .map(|value| (coin_type.to_string(), value.to_string()))
            })
            .collect();

        let value = Profile {
            name: name.to_string(),
            address: address.and_then(|it| EIP55Address::from_str(it.as_str()).ok()),
            avatar,
            header,
            display,
            records,
            chains,
            fresh: chrono::offset::Utc::now().timestamp_millis(),
            resolver: EIP55Address(resolved.resolver),
            ccip_urls: resolved.ccip_urls,
            errors: resolved
                .invalid
                .iter()
                .map(|(key, value)| (key.name(), value.to_string()))
                .collect(),
        };

        let response =
            serde_json::to_string(&value).map_err(|err| ProfileError::Other(err.to_string()))?;

        self.cache
            .set(&cache_key, &response, 600)
            .await
            .map_err(|CacheError::Other(err)| {
                ProfileError::Other(format!("cache set failed: {}", err))
            })?;

        Ok(value)
    }
}
