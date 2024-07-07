pub mod agent;
pub mod env;
pub mod trade;

lazy_static::lazy_static! {
    pub static ref AGENT_ENV: env::AgentEnv = env::AgentEnv::new();
}
