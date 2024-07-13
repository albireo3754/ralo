use agent::{trade::BinanaceTradeService, AGENT_ENV};
use binance::{account::Account, api::Binance, config::Config, margin::Margin};
use ralo_core::{entity::symbol::Symbol, log::init_log_settings};

#[tokio::main]
async fn main() {
    init_log_settings();

    let account = Account::new_with_config(
        Some(AGENT_ENV.key.to_owned()),
        Some(AGENT_ENV.secret.to_owned()),
        &Config::default(),
    );
    let client = BinanaceTradeService::new(account);

    let result = client.fetch_account_info().await;
    // info!("{:?}", result);
}
