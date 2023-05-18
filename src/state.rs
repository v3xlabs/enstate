use ethers::providers::{Http, Provider};
use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub redis: ConnectionManager,
    pub provider: Provider<Http>,
}
