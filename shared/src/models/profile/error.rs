use ethers::providers::ProviderError;
use thiserror::Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("Not Found")]
    NotFound,

    #[error("RPC error {0}")]
    RPCError(#[from] ProviderError),

    #[error("DNS encode error: {0}")]
    DNSEncodeError(String),

    #[error("Implementation error: {0}")]
    ImplementationError(String),

    #[error("Other: {0}")]
    Other(String),
}
