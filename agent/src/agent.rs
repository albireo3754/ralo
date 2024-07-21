use tracing::{info, instrument};

use crate::{
    algorithm::{
        common::{Algorithm, Choice},
        mock::MockAlgorithm,
    },
    history::TradeHistoryService,
    state::{RlState, RlStateInternal},
    trade::{TradeApprovalService, TradeResult, TradeService},
};

pub struct Agent<T, C, A, H>
where
    T: TradeService,
    C: TradeApprovalService,
    A: Algorithm,
    H: TradeHistoryService,
{
    pub state: RlState,
    pub algorithm: A,
    pub trade_approval: C,
    pub trade_service: T,
}

impl<T, C, A, H> Agent<T, C, A, H>
where
    T: TradeService,
    C: TradeApprovalService,
    A: Algorithm,
    H: TradeHistoryService,
{
    pub fn buy(&self) {
        println!("buy");
    }

    pub fn sell(&self) {
        println!("sell");
    }

    pub async fn run(mut self) {
        loop {
            let state = self.state.get().await;

            // self.handle_state(&state)
            // history
        }
    }

    #[instrument(skip(self, state))]
    pub fn handle_state(&self, state: &RlStateInternal) {
        info!("handle_state");
        // trade 판단

        // trade 판단 auto or manual 에 따라서 에이전트 시스템에게 확인받고 오기
        let choices = self.algorithm.decision(state);
        let approved = self.trade_approval.check_trade_approve(&choices);
        let trade_result = if approved {
            self.trade_service.trade(&choices)
        } else {
            TradeResult {}
        };

        // TODO: - record choice & agent history

        // self.trade

        // new loop
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // #[tokio::test]
    // async fn agent_works() {
    //     let _agent = Agent {trade_service:MockTradeService{}, state: Default::default() }
    //     };
    // }

    // #[tokio::test]
    // async fn agent_buy() {
    //     let agent = Agent {
    //         trade_service: MockTradeService {},
    //         state: Default::default()
    //     };
    //     agent.buy();
    // }

    // #[tokio::test]
    // async fn agent_sell() {
    //     let agent = Agent {
    //         trade_service: MockTradeService {},
    //         state: Default::default()
    //     };
    //     agent.sell();
    // }
}
