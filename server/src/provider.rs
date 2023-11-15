use ethers::providers::{Http, Provider};

#[derive(Clone)]
pub struct RoundRobinProvider {
    __last_index: usize,
    providers: Vec<Provider<Http>>,
}

impl RoundRobinProvider {
    pub fn new(rpc_urls: Vec<String>) -> Self {
        Self {
            __last_index: 0,
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
        unsafe {
            (*(self as *const Self).cast_mut()).__last_index =
                (self.__last_index + 1) % self.providers.len();
        }

        self.providers.get(self.__last_index)
    }
}
