use std::fmt;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, State, Error>;

#[derive(Clone, Debug, Default)]
pub struct State {
    // pub id: i32,
    // pub player_manager: PlayerManager<SupabaseDBManager>,
} // User data, which is stored and accessible in all command invocations

#[derive(Debug)]
pub struct CustomError {
    details: String,
}

impl CustomError {
    pub fn new(msg: &str) -> CustomError {
        CustomError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for CustomError {
    fn description(&self) -> &str {
        &self.details
    }
}
