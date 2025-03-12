use std::collections::HashMap;

use axum::async_trait;
use enstate_shared::core::Profile;
use enstate_shared::discovery::Discovery;
use ethers::providers::namehash;
use serde::{Deserialize, Serialize};

pub struct DiscoveryEngine {
    client: meilisearch_sdk::client::Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeiliProfileDocument {
    name_hash: String,
    name: String,
    avatar: Option<String>,
    header: Option<String>,
    display: String,
    addresses: HashMap<String, String>,
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
            addresses: profile.chains.iter().map(|(chain, address)| (chain.to_string(), address.to_string())).collect(),
            fresh: profile.fresh,
        }
    }
}

impl DiscoveryEngine {
    pub fn new(url: &str, key: Option<&str>) -> Self {
        Self {
            client: meilisearch_sdk::client::Client::new(url, key).unwrap(),
        }
    }

    pub async fn create_table_if_not_exists(&self) -> Result<(), ()> {
        

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

    async fn query_search(&self, query: String) -> Result<Vec<Profile>, ()> {
        todo!()
    }
}
