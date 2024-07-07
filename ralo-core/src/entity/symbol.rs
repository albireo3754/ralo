use strum::AsRefStr;

#[derive(Debug, Clone, AsRefStr)]
pub enum Symbol {
    #[strum(serialize = "btcusdt")]
    Bitcoin,
    #[strum(serialize = "ethusdt")]
    Ethereum,
    #[strum(serialize = "fdusdusdt")]
    TetherUS,
    #[strum(serialize = "bnbusdt")]
    Bnb,
    #[strum(serialize = "solusdt")]
    Solana,
    #[strum(serialize = "xrpusdt")]
    Ripple,
    #[strum(serialize = "usdcusdt")]
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
