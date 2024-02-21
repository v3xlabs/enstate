use std::collections::HashMap;
use std::sync::Arc;

use ethers::middleware::MiddlewareBuilder;
use ethers::prelude::Address;
use ethers_ccip_read::CCIPReadMiddleware;

use crate::core::error::ProfileError;
use crate::core::lookup_data::LookupInfo;
use crate::core::ENSService;
use crate::models::lookup::{ENSLookup, ENSLookupError, LookupState};

use super::universal_resolver::resolve_universal;

pub struct ResolvedCalldata {
    pub resolver: Address,
    pub ccip_urls: Vec<String>,
    pub records: HashMap<ENSLookup, String>,
    pub invalid: HashMap<ENSLookup, ENSLookupError>,
}

impl ENSService {
    // TODO: per-record caching
    pub async fn resolve_records(
        &self,
        lookup: LookupInfo,
        calldata: &[ENSLookup],
        fresh: bool,
    ) -> Result<ResolvedCalldata, ProfileError> {
        let name = match lookup {
            LookupInfo::Name(name) => name,
            LookupInfo::Address(address) => self.primary_from_address(&address, fresh).await?,
        };

        // let cache_key = format!("n:{name}");

        let rpc = self.rpc.get_instance();

        let rpc = rpc.wrap_into(CCIPReadMiddleware::new);

        // If the value is in the cache, return it
        // if !fresh {
        //     if let Ok(value) = self.cache.get(&cache_key).await {
        //         if value.is_empty() {
        //             return Err(ResolveError::NotFound);
        //         }
        //
        //         let entry_result: Result<Profile, _> = serde_json::from_str(value.as_str());
        //         if let Ok(entry) = entry_result {
        //             return Ok(entry);
        //         }
        //     }
        // }

        let rpc = Arc::new(rpc);

        // ENS CCIP unwrapper is limited to 50 sub-requests, i.e. per request
        let mut resolves = Vec::new();

        for chunk in calldata.chunks(50) {
            resolves.push(resolve_universal(&name, chunk, &rpc, &self.universal_resolver).await?);
        }

        let Some((_, resolver, ccip_urls)) = resolves.first() else {
            return Err(ProfileError::ImplementationError(String::new()));
        };

        let data = resolves
            .iter()
            .flat_map(|(data, _, _)| data)
            .collect::<Vec<_>>();

        let mut results: HashMap<ENSLookup, String> = HashMap::new();
        let mut errors: HashMap<ENSLookup, ENSLookupError> = HashMap::default();

        let lookup_state = LookupState {
            rpc,
            opensea_api_key: self.opensea_api_key.clone(),
            ipfs_gateway: self.ipfs_gateway.clone(),
        };

        // Assume results & calldata have the same length
        // Look through all calldata and decode the results at the same index
        for (index, calldata) in calldata.iter().enumerate() {
            let res = data[index];
            // TODO: think about this
            //  current behaviour ignores all errors from a resolver
            let result = if res.success {
                calldata.decode(&res.data, &lookup_state).await
            } else {
                Ok(String::new())
            };

            match result {
                Ok(result) if !result.is_empty() => {
                    results.insert(calldata.clone(), result);
                }
                Err(error) if !matches!(error, ENSLookupError::CCIPError { .. }) => {
                    errors.insert(calldata.clone(), error);
                }
                _ => {}
            }
        }

        let value = ResolvedCalldata {
            resolver: *resolver,
            ccip_urls: ccip_urls.clone(),
            records: results,
            invalid: errors,
        };

        // let response = serde_json::to_string(&value)
        //     .map_err(|err| ProfileResolveError::Other(err.to_string()))?;
        //
        // self.cache
        //     .set(&cache_key, &response, 600)
        //     .await
        //     .map_err(|CacheError::Other(err)| {
        //         ProfileResolveError::Other(format!("cache set failed: {}", err))
        //     })?;

        Ok(value)
    }

    // utility function
    pub async fn resolve_record_simple(
        &self,
        lookup: LookupInfo,
        record: ENSLookup,
        fresh: bool,
    ) -> Result<String, ProfileError> {
        let resolved = self
            .resolve_records(lookup, &[record.clone()], fresh)
            .await?;

        let record = resolved
            .records
            .get(&record)
            .ok_or_else(|| ProfileError::NotFound)?;

        Ok(record.to_string())
    }
}
