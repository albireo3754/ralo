use agent::{trade::BinanaceTradeService, AGENT_ENV};
use binance::{account::Account, api::Binance, config::Config};
use ralo_core::entity::symbol::Symbol;

#[tokio::main]
async fn main() {
    let account = Account::new_with_config(
        Some(AGENT_ENV.key.to_owned()),
        Some(AGENT_ENV.secret.to_owned()),
        &Config::default(),
    );
    let client = BinanaceTradeService::new(account);

    let result = client.buy(Symbol::Bitcoin, 1.0, 1.0).await;
    println!("{:?}", result);
}
