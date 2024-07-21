pub mod agent;
pub mod algorithm;
pub mod env;
pub mod history;
pub mod state;
pub mod trade;

lazy_static::lazy_static! {
    pub static ref AGENT_ENV: env::AgentEnv = env::AgentEnv::new();
}
