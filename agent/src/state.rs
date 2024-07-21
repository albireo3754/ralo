use std::{collections::BTreeMap, sync::Arc};

use ralo_core::entity::symbol::Symbol;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct RlState {
    state: Arc<RwLock<RlStateInternal>>,
}

impl RlState {
    pub fn new() -> RlState {
        RlState {
            state: Default::default(),
        }
    }

    pub async fn get(&self) -> RlStateInternal {
        (*self.state.read().await).clone()
    }
}

#[derive(Default, Clone)]
pub struct RlStateInternal {
    pub portflio: Portfolio,
    pub market: Market,
}

#[derive(Default, Clone)]
pub struct Portfolio {
    pub items: BTreeMap<Symbol, PortfolioItem>,
}

#[derive(Clone)]
pub struct PortfolioItem {
    pub symbol: Symbol,
    pub average: f64,
    pub ratio: f64,
    pub count: f64,
}

#[derive(Default, Clone)]
pub struct Market {
    pub spots: BTreeMap<Symbol, Spot>,
}

#[derive(Clone)]
pub struct Spot {
    pub symbol: Symbol,
    pub price_history: Vec<i64>,
}
