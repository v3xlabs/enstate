use std::collections::HashMap;

use axum::async_trait;
use enstate_shared::core::lookup_data::LookupInfo;
use enstate_shared::core::{ENSService, Profile};
use enstate_shared::discovery::{Discovery, SearchResult};
use ethers::providers::namehash;
use futures::future::join_all;
use serde::{Deserialize, Serialize};

pub struct DiscoveryEngine {
    client: meilisearch_sdk::client::Client,
    project_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeiliProfileDocument {
    name_hash: String,
    name: String,
    avatar: Option<String>,
    header: Option<String>,
    display: String,
    bio: Option<String>,
    addresses: Option<HashMap<String, String>>,
    records: Option<HashMap<String, String>>,
    fresh: i64,
}

impl From<&Profile> for MeiliProfileDocument {
    fn from(profile: &Profile) -> Self {
        Self {
            name_hash: format!("{:x}", namehash(&profile.name)),
            name: profile.name.clone(),
            avatar: profile.avatar.clone(),
            header: profile.header.clone(),
            display: profile.display.clone(),
            bio: profile.records.get("bio").cloned(),
            addresses: if profile.chains.is_empty() { 
                None 
            } else {
                Some(profile.chains.iter().map(|(chain, address)| (chain.to_string(), address.to_string())).collect())
            },
            records: if profile.records.is_empty() {
                None
            } else {
                Some(profile.records.clone().into_iter().collect())
            },
            fresh: profile.fresh,
        }
    }
}

impl DiscoveryEngine {
    pub fn new(url: &str, key: Option<&str>, project_id: String) -> Self {
        Self {
            client: meilisearch_sdk::client::Client::new(url, key).unwrap(),
            project_id,
        }
    }

    pub async fn create_table_if_not_exists(&self) -> Result<(), ()> {
        // Use the project_id in the method
        let index = self.client.index(&self.project_id);
        let _ = index.create().await;

        Ok(())
    }
}

#[async_trait]
impl Discovery for DiscoveryEngine {
    async fn discover_name(&self, profile: &Profile) -> Result<(), ()> {
        let ccip_urls: Vec<String> = profile.ccip_urls.iter()
            .map(|url| url.to_string())
            .collect();
            
        let errors: Vec<String> = profile.errors.iter()
            .map(|(key, val)| format!("{}: {}", key, val))
            .collect();
            
        // let query = self.client
        //     .query("INSERT INTO enstate.profiles (name, address, avatar, header, display, contenthash, resolver, ccip_urls, errors, fresh) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        //     .bind(profile.name.clone())
        //     .bind(profile.address.as_ref().map(|x| x.to_string()).unwrap_or_default())
        //     .bind(profile.avatar.as_deref().unwrap_or_default())
        //     .bind(profile.header.as_deref().unwrap_or_default())
        //     .bind(profile.display.clone())
        //     .bind(profile.contenthash.as_deref().unwrap_or_default())
        //     .bind(profile.resolver.clone())
        //     .bind(ccip_urls)
        //     .bind(errors)
        //     .bind(profile.fresh);

        let document = MeiliProfileDocument::from(profile);

        let documents = vec![document];
        
        let x = self.client.index("profiles");
        let x = x.add_documents(&documents, Some("name_hash")).await;

        match x {
            Ok(result) => {
                tracing::info!("Inserted profile: {:?}", result);
                Ok(())
            },
            Err(e) => {
                tracing::error!("Error inserting profile: {}", e);
                Err(())
            }
        }
    }

    async fn query_search(&self, service: &ENSService, query: String) -> Result<Vec<SearchResult>, ()> {
        let index = self.client.index("profiles");
        
        // Create search with query and limit to 12 results
        let search = index.search()
            .with_query(&query)
            .with_limit(12)
            .build();
            
        // Execute the search
        match search.execute::<MeiliProfileDocument>().await {
            Ok(search_results) => {
                tracing::info!("Search results: found {} matches", search_results.hits.len());
                
                // Return empty vector if no results
                if search_results.hits.is_empty() {
                    return Ok(vec![]);
                }

                // Extract the name for each result to use with resolve_name
                let names: Vec<SearchResult> = search_results.hits
                    .into_iter()
                    .map(|hit| SearchResult { name: hit.result.name })
                    .collect();

                Ok(names)
            },
            Err(e) => {
                tracing::error!("Error searching profiles: {}", e);
                Err(())
            }
        }
    }
}
