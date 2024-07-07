use crate::trade::TradeService;

struct Agent<T>
where
    T: TradeService,
{
    trade_service: T,
}

impl<T> Agent<T>
where
    T: TradeService,
{
    pub fn buy(&self) {
        println!("buy");
    }

    pub fn sell(&self) {
        println!("sell");
    }
}

#[cfg(test)]
mod tests {
    use crate::trade::mock::MockTradeService;

    use super::*;

    #[tokio::test]
    async fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn agent_works() {
        let _agent = Agent {
            trade_service: MockTradeService {},
        };
    }

    #[tokio::test]
    async fn agent_buy() {
        let agent = Agent {
            trade_service: MockTradeService {},
        };
        agent.buy();
    }

    #[tokio::test]
    async fn agent_sell() {
        let agent = Agent {
            trade_service: MockTradeService {},
        };
        agent.sell();
    }
}
