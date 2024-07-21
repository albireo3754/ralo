struct BackTestService {
    // trade service
    // backtracking history service
}

impl BackTestService {
    pub fn new() -> Self {
        BackTestService {}
    }

    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                    println!("Backtesting...");
                    // handle_state(&state);
                }
                // env stream => { portfolio 갱신 & trading }
                _ = tokio::signal::ctrl_c() => {
                    println!("Exiting...");
                    break;
                }
            }
        }
        // 1. Get all the strategies
        // 2. Get all the stocks
        // 3. Get all the trading days
        // 4. Iterate through all the strategies, stocks, and trading days to backtest
        // 5. Record the backtesting results
    }

    // pub async fn handle_state(state: &State) {}
}
