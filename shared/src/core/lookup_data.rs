use ethers::prelude::Address;
use thiserror::Error;

#[derive(Debug)]
pub enum LookupInfo {
    Name(String),
    Address(Address),
}

#[derive(Error, Clone, Debug)]
pub enum NameParseError {
    #[error("Invalid name format")]
    InvalidNameFormat,
}

impl LookupInfo {
    pub fn guess<T: AsRef<str>>(name_or_address: T) -> Result<LookupInfo, NameParseError> {
        let name_or_address = name_or_address.as_ref();

        if let Ok(address) = name_or_address.parse::<Address>() {
            return Ok(LookupInfo::Address(address));
        }

        if !crate::patterns::test_domain(name_or_address) {
            return Err(NameParseError::InvalidNameFormat);
        }

        Ok(LookupInfo::Name(name_or_address.to_lowercase()))
    }
}
