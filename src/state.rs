use ethers::providers::{Http, Provider};
use ethers_ccip_read::CCIPReadMiddleware;
use redis::aio::ConnectionManager;

#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct AppState {
    pub redis: ConnectionManager,
    pub fallback_provider: Provider<Http>,
    pub provider: CCIPReadMiddleware<Provider<Http>>,
}
