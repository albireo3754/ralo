use agent::{trade::BinanaceTradeService, AGENT_ENV};
use binance::{account::Account, api::Binance, config::Config};
use ralo_core::{entity::symbol::Symbol, log::init_log_settings};
use tracing::info;

#[tokio::main]
async fn main() {
    init_log_settings();

    let account = Account::new_with_config(
        Some(AGENT_ENV.key.to_owned()),
        Some(AGENT_ENV.secret.to_owned()),
        &Config::default(),
    );
    let client = BinanaceTradeService::new(account);

    let result = client.buy(Symbol::Bitcoin, 1.0, 1.0).await;
    info!("{:?}", result);
}
