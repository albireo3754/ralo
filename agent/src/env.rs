use dotenv::dotenv;
use ralo_core::utils::env::get_env_string;

pub struct AgentEnv {
    pub key: String,
    pub secret: String,
}

impl AgentEnv {
    pub fn new() -> Self {
        dotenv().ok();
        let key = get_env_string("BINANCE_API_KEY");
        let secret = get_env_string("BINANCE_API_SECRET");

        AgentEnv { key, secret }
    }
}

impl Default for AgentEnv {
    fn default() -> Self {
        Self::new()
    }
}
