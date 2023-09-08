use self::{coins::CoinType, slip44::SLIP44, evm::ChainId};

pub mod evm;
pub mod slip44;
pub mod coins;

pub struct Coins {
    pub coins: Vec<CoinType>,
}

impl Default for Coins {
    fn default() -> Self {
        Self {
            coins: vec![
                SLIP44::Bitcoin.into(),
                SLIP44::Litecoin.into(),
                SLIP44::Hedera.into(),
                SLIP44::Stellar.into(),
                ChainId::Polygon.into(),
            ]
        }
    }
}
