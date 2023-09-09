use self::{coins::CoinType, evm::ChainId, slip44::SLIP44};

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
                SLIP44::Ethereum.into(),
                SLIP44::EthereumClassic.into(),
                SLIP44::Solana.into(),
                SLIP44::Binance.into(),
                SLIP44::Dogecoin.into(),
                SLIP44::Monero.into(),
                ChainId::Polygon.into(),
                ChainId::Optimism.into(),
            ]
        }
    }
}
