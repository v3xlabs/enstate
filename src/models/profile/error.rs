use ethers::providers::ProviderError;
use thiserror::Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("Not Found")]
    NotFound,

    #[error("RPC Error {0}")]
    RPCError(#[from] ProviderError),

    #[error("Unknown")]
    Unknown,
}
