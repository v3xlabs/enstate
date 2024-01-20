use std::fmt::{Display, Formatter};

use ethers_core::types::U256;

use super::{evm::ChainId, slip44::SLIP44};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum CoinType {
    Slip44(SLIP44),
    Evm(ChainId),
}

impl From<CoinType> for U256 {
    fn from(value: CoinType) -> Self {
        match value {
            CoinType::Slip44(slip44) => slip44.into(),
            CoinType::Evm(chain) => chain.as_ensip11().into(),
        }
    }
}

impl From<u64> for CoinType {
    fn from(value: u64) -> CoinType {
        if value >= 0x8000_0000 {
            return CoinType::Evm(ChainId::from(value & (!0x8000_0000)));
        }

        CoinType::Slip44(SLIP44::from(value as u32))
    }
}

impl Display for CoinType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let coin_name = match self {
            Self::Slip44(slip44) => slip44.to_string(),
            Self::Evm(chain) => chain.to_string(),
        };

        f.write_str(coin_name.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{evm::ChainId, slip44::SLIP44};
    use super::*;

    #[test]
    fn test_coin_type() {
        let coin_type: CoinType = SLIP44::Bitcoin.into();
        let coin_type: U256 = coin_type.into();
        assert_eq!(coin_type, 0.into());
    }

    #[test]
    fn test_coin_type_evm() {
        let coin_type: CoinType = ChainId::Ethereum.into();
        let coin_type: U256 = coin_type.into();

        assert_eq!(coin_type.to_string(), "2147483649".to_string());
    }

    #[test]
    fn test_coin_type_evm_gnosis() {
        let coin_type: CoinType = ChainId::Gnosis.into();
        let coin_type: U256 = coin_type.into();

        assert_eq!(coin_type.to_string(), "2147483748".to_string());
    }

    #[test]
    fn test_to_coin_type() {
        let coin_type: CoinType = CoinType::from(0);

        assert_eq!(coin_type, CoinType::Slip44(SLIP44::Bitcoin));
    }

    #[test]
    fn test_to_coin_type_evm() {
        let coin_type: CoinType = CoinType::from(1);

        assert_eq!(coin_type, CoinType::Evm(ChainId::Ethereum));
    }

    #[test]
    fn test_to_coin_type_evm_gnosis() {
        let coin_type: CoinType = CoinType::from(2147483658);

        assert_eq!(coin_type, CoinType::Evm(ChainId::Gnosis));
    }
}
