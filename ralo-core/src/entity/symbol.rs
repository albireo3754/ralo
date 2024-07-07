use strum::AsRefStr;

#[derive(Debug, Clone, AsRefStr)]
pub enum Symbol {
    #[strum(serialize = "BTCUSDT")]
    Bitcoin,
    #[strum(serialize = "ETHUSDT")]
    Ethereum,
    #[strum(serialize = "FDUSDUSDT")]
    TetherUS,
    #[strum(serialize = "BNBUSDT")]
    Bnb,
    #[strum(serialize = "SOLUSDT")]
    Solana,
    #[strum(serialize = "XRPUSDT")]
    Ripple,
    #[strum(serialize = "USDCUSDT")]
    USDCoin,
    #[strum(serialize = "adausdt")]
    Cardano,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_to_str_works() {
        assert_eq!(Symbol::Bitcoin.as_ref(), "btcusdt");
    }
}
