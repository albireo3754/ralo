pub mod mock;

use anyhow::Result;
use binance::{
    account::{Account, OrderRequest},
    rest_model::OrderSide,
};
use ralo_core::entity::symbol::Symbol;

// https://binance-docs.github.io/apidocs/spot/en/#sub-account-endpoints
// https://binance.github.io/binance-api-swagger/

// Bitcoin("btcusdt"),
// Ethereum("ethusdt"),
// TetherUS("fdusdusdt"),
// BNB("bnbusdt"),
// Solana("solusdt"),
// Ripple("xrpusdt"),
// USDCoin("usdcusdt"),
// Cardano("adausdt"),
// Unknown("unknown");

pub trait TradeService {}

pub struct BinanaceTradeService {
    account: Account,
}

impl BinanaceTradeService {
    pub fn new(account: Account) -> Self {
        Self { account }
    }

    pub async fn buy(&self, symbol: Symbol, quantity: f64, price: f64) -> Result<()> {
        let order = OrderRequest {
            symbol: symbol.as_ref().to_owned(),
            side: OrderSide::Buy,
            time_in_force: None,
            quantity: Some(quantity),
            price: Some(price),
            ..Default::default()
        };

        let _ = self
            .account
            .place_test_order(order)
            .await
            .map_err(|e| anyhow::anyhow!("buy error: {}", e))?;

        Ok(())
    }
}

impl TradeService for BinanaceTradeService {}
