use ethers::providers::{Http, Provider};
use ethers_ccip_read::CCIPReadMiddleware;
use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub redis: ConnectionManager,
    pub provider: CCIPReadMiddleware<Provider<Http>>,
    pub fallback_provider: Provider<Http>,
}
