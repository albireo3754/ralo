use agent::{
    agent::Agent,
    trade::{mock::MockTradeService, BinanaceTradeService},
    AGENT_ENV,
};
use binance::{account::Account, api::Binance, config::Config};
use ralo_core::log::init_log_settings;

#[tokio::main]
async fn main() {
    init_log_settings();

    let account = Account::new_with_config(
        Some(AGENT_ENV.key.to_owned()),
        Some(AGENT_ENV.secret.to_owned()),
        &Config::default(),
    );
    let client = BinanaceTradeService::new(account);

    let agent = Agent {
        state: Default::default(),
        trade_service: MockTradeService::new(),
    };

    agent.run().await;
    // info!("{:?}", result);
}
