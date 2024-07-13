pub mod mock;

use std::io::Write;

use anyhow::Result;
use binance::{
    account::{Account, OrderRequest},
    api::Binance,
    rest_model::OrderSide,
};
use ralo_core::entity::symbol::Symbol;
use tracing::{info, instrument};

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

    pub async fn fetch_account_info(&self) -> Result<()> {
        let mut account = self.account.get_account().await.map_err(|e| {
            tracing::error!("buy error: {:#?}", e);
            anyhow::anyhow!("buy error: {}", e)
        })?;

        info!("{:#?}", account);

        // filter
        account.balances.retain(|b| b.free > 0.0 || b.locked > 0.0);

        let json = serde_json::to_string_pretty(&account).map_err(|e| {
            tracing::error!("buy error: {:#?}", e);
            anyhow::anyhow!("buy error: {}", e)
        })?;
        // write test.json
        let mut file = std::fs::File::create("test.json").map_err(|e| {
            tracing::error!("buy error: {:#?}", e);
            anyhow::anyhow!("buy error: {}", e)
        })?;

        file.write_all(json.as_bytes()).map_err(|e| {
            tracing::error!("buy error: {:#?}", e);
            anyhow::anyhow!("buy error: {}", e)
        })?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn buy(&self, symbol: Symbol, quantity: f64, price: f64) -> Result<()> {
        info!("buy symbol: {}", symbol.as_ref().to_owned());
        let order = OrderRequest {
            symbol: symbol.as_ref().to_owned(),
            side: OrderSide::Buy,
            time_in_force: None,
            quantity: Some(quantity),
            price: Some(price),
            ..Default::default()
        };

        let _ = self.account.place_test_order(order).await.map_err(|e| {
            tracing::error!("buy error: {}", e);
            anyhow::anyhow!("buy error: {}", e)
        })?;

        // let _ = self.account.place_order(order).await.map_err(|e| {
        //     tracing::error!("buy error: {}", e);
        //     anyhow::anyhow!("buy error: {}", e)
        // })?;

        Ok(())
    }
}

impl TradeService for BinanaceTradeService {}
