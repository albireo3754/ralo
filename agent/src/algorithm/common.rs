use ralo_core::entity::symbol::Symbol;

use crate::state::RlStateInternal;

pub enum ChoiceType {
    Buy,
    Sell,
    Hold,
}

pub struct Choice {
    pub r#type: ChoiceType,
    pub symbol: Symbol,
    pub count: f64,
    pub price: f64,
}
pub trait Algorithm {
    fn decision(&self, _state: &RlStateInternal) -> TradeChoices {
        Vec::new()
    }
}

pub type TradeChoices = Vec<Choice>;
