use super::TradeService;

pub struct MockTradeService {}

impl MockTradeService {
    pub fn new() -> Self {
        MockTradeService {}
    }
}

impl Default for MockTradeService {
    fn default() -> Self {
        Self::new()
    }
}

impl TradeService for MockTradeService {}
