use ethers::providers::{Http, Provider};

#[derive(Clone)]
pub struct AppState {
    pub provider: Provider<Http>,
}
