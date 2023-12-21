use ethers::prelude::Http;
use ethers::providers::{Provider, ProviderError};
use ethers_ccip_read::CCIPReadMiddlewareError;
use std::sync::Arc;
use thiserror::Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("Not Found")]
    NotFound,

    #[error("RPC error: {0}")]
    RPCError(#[from] ProviderError),

    #[error("CCIP error: {0}")]
    CCIPError(#[from] CCIPReadMiddlewareError<Arc<Provider<Http>>>),

    #[error("DNS encode error: {0}")]
    DNSEncodeError(String),

    #[error("Implementation error: {0}")]
    ImplementationError(String),

    #[error("Other: {0}")]
    Other(String),
}

impl AsRef<ProfileError> for ProfileError {
    fn as_ref(&self) -> &ProfileError {
        self
    }
}
