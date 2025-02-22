use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use ethers_core::types::Address;
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(Debug, Copy, Clone)]
pub enum RSKIPChain {
    Ethereum,
    Other(u64),
}

impl From<RSKIPChain> for u64 {
    fn from(value: RSKIPChain) -> Self {
        match value {
            RSKIPChain::Ethereum => 0,
            RSKIPChain::Other(val) => val,
        }
    }
}

#[derive(SerializeDisplay, DeserializeFromStr, Clone, PartialEq)]
pub struct EIP55Address(pub Address);

impl Display for EIP55Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "0x{}",
            encode_rskip60(self.0.as_bytes(), RSKIPChain::Ethereum)
        )
    }
}

impl Debug for EIP55Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "0x{}",
            encode_rskip60(self.0.as_bytes(), RSKIPChain::Ethereum)
        )
    }
}

impl FromStr for EIP55Address {
    type Err = rustc_hex::FromHexError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Address::from_str(s).map(EIP55Address)
    }
}

pub fn encode_rskip60(data: &[u8], chain: RSKIPChain) -> String {
    let raw = hex::encode(data).to_ascii_lowercase();
    if data.len() > 20 {
        return raw;
    }
    let hash = ethers::utils::keccak256(format!(
        "{:}{raw}",
        match chain {
            RSKIPChain::Ethereum => String::new(),
            RSKIPChain::Other(_) => format!("{}0x", u64::from(chain)),
        }
    ));

    return raw
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if hash[i >> 1] << ((i & 1) << 2) >= 0x80 {
                c.to_ascii_uppercase()
            } else {
                c
            }
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rskip60_eth() {
        assert_eq!(
            encode_rskip60(
                hex::decode("2b5c7025998f88550ef2fece8bf87935f542c190")
                    .unwrap()
                    .as_slice(),
                RSKIPChain::Ethereum
            ),
            "2B5c7025998f88550Ef2fEce8bf87935f542C190".to_string()
        );

        assert_eq!(
            encode_rskip60(
                hex::decode("77c0f878fde3d1ad2830ca9e46d7c47c951a93fb")
                    .unwrap()
                    .as_slice(),
                RSKIPChain::Ethereum
            ),
            "77c0F878FdE3d1ad2830ca9E46D7c47C951A93fb".to_string()
        );
    }

    #[test]
    fn test_rskip60_rootstock() {
        assert_eq!(
            encode_rskip60(
                hex::decode("5aaeb6053f3e94c9b9a09f33669435e7ef1beaed")
                    .unwrap()
                    .as_slice(),
                RSKIPChain::Other(30)
            ),
            "5aaEB6053f3e94c9b9a09f33669435E7ef1bEAeD".to_string()
        );

        assert_eq!(
            encode_rskip60(
                hex::decode("fb6916095ca1df60bb79ce92ce3ea74c37c5d359")
                    .unwrap()
                    .as_slice(),
                RSKIPChain::Other(30)
            ),
            "Fb6916095cA1Df60bb79ce92cE3EA74c37c5d359".to_string()
        );
    }

    #[test]
    fn test_eip55address_display() {
        assert_eq!(
            EIP55Address(Address::from_str("0x2b5c7025998f88550ef2fece8bf87935f542c190").unwrap())
                .to_string(),
            "0x2B5c7025998f88550Ef2fEce8bf87935f542C190".to_string()
        );
    }
}
