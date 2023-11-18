use ethers::providers::{Http, Provider};
use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct RoundRobin {
    providers: Vec<Provider<Http>>,
}

impl RoundRobin {
    pub fn new(rpc_urls: Vec<String>) -> Self {
        Self {
            providers: rpc_urls
                .into_iter()
                .map(|rpc_url| {
                    Provider::<Http>::try_from(rpc_url).expect("rpc_url should be a valid URL")
                })
                .collect(),
        }
    }

    // returns a random rpc provider
    pub fn get_provider(&self) -> Option<&Provider<Http>> {
        self.providers.choose(&mut rand::thread_rng())
    }
}
