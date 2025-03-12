use axum::async_trait;
use enstate_shared::core::Profile;
use enstate_shared::discovery::Discovery;

pub struct DiscoveryEngine {
    client: clickhouse::Client,
}

impl DiscoveryEngine {
    pub fn new(url: &str, user: &str, password: &str) -> Self {
        Self {
            client: clickhouse::Client::default().with_url(url).with_user(user).with_password(password),
        }
    }

    pub async fn create_table_if_not_exists(&self) -> Result<(), ()> {
        // Create the profiles table
        let profiles_table_query = self.client.query("CREATE TABLE IF NOT EXISTS enstate.profiles (name String, address String, avatar String, header String, display String, contenthash String, resolver String, ccip_urls Array(String), errors Array(String), fresh Int64) ENGINE = ReplacingMergeTree(fresh) ORDER BY name");

        match profiles_table_query.execute().await {
            Ok(_) => {},
            Err(e) => {
                tracing::error!("Error creating profiles table: {}", e); 
                return Err(());
            }
        };
        
        // Create the stats aggregation table
        let stats_table_query = self.client.query("
            CREATE TABLE IF NOT EXISTS enstate.profile_stats_agg
            (
                day                 Date,
                total_profiles      UInt64,
                profiles_with_avatar UInt64,
                profiles_with_header UInt64,
                profiles_with_display UInt64,
                profiles_with_contenthash UInt64
            )
            ENGINE = SummingMergeTree
            PARTITION BY day
            ORDER BY day
        ");
        
        match stats_table_query.execute().await {
            Ok(_) => {},
            Err(e) => {
                tracing::error!("Error creating profile_stats_agg table: {}", e); 
                return Err(());
            }
        };
        
        // Create the materialized view
        let materialized_view_query = self.client.query("
            CREATE MATERIALIZED VIEW IF NOT EXISTS enstate.mv_profile_stats_agg
            TO enstate.profile_stats_agg
            AS
            SELECT
                toDate(fromUnixTimestamp64Milli(fresh)) AS day,
                count() AS total_profiles,
                countIf(avatar != '') AS profiles_with_avatar,
                countIf(header != '') AS profiles_with_header,
                countIf(display != '') AS profiles_with_display,
                countIf(contenthash != '') AS profiles_with_contenthash
            FROM enstate.profiles
            GROUP BY day
        ");
        
        match materialized_view_query.execute().await {
            Ok(_) => {},
            Err(e) => {
                tracing::error!("Error creating materialized view: {}", e); 
                return Err(());
            }
        };

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
            
        let query = self.client
            .query("INSERT INTO enstate.profiles (name, address, avatar, header, display, contenthash, resolver, ccip_urls, errors, fresh) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(profile.name.clone())
            .bind(profile.address.as_ref().map(|x| x.to_string()).unwrap_or_default())
            .bind(profile.avatar.as_deref().unwrap_or_default())
            .bind(profile.header.as_deref().unwrap_or_default())
            .bind(profile.display.clone())
            .bind(profile.contenthash.as_deref().unwrap_or_default())
            .bind(profile.resolver.clone())
            .bind(ccip_urls)
            .bind(errors)
            .bind(profile.fresh);

        match query.execute().await {
            Ok(_) => Ok(()),
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
