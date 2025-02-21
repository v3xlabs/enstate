use std::sync::Arc;

use enstate_shared::utils::factory::Factory;
use ethers::providers::{Http, Provider};
use rand::seq::SliceRandom;
use tracing::warn;

#[derive(Clone)]
pub struct RoundRobin {
    providers: Vec<Arc<Provider<Http>>>,
}

impl RoundRobin {
    pub fn new(rpc_urls: Vec<String>) -> Self {
        Self {
            providers: rpc_urls
                .into_iter()
                .filter_map(|rpc_url| {
                    let provider = Provider::<Http>::try_from(&rpc_url);
                    if let Err(err) = provider {
                        warn!("provider {rpc_url} is not valid: {err}");
                    }

                    provider.ok().map(Arc::new)
                })
                .collect(),
        }
    }
}

impl Factory<Arc<Provider<Http>>> for RoundRobin {
    fn get_instance(&self) -> Arc<Provider<Http>> {
        self.providers
            .choose(&mut rand::thread_rng())
            .expect("provider should exist")
            .clone()
    }
}
