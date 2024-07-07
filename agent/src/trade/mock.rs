use crate::trade::TradeService;

pub struct MockTradeService {}

impl MockTradeService {
    fn new() -> Self {
        MockTradeService {}
    }
}

impl TradeService for MockTradeService {}
