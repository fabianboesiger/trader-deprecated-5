mod coin;
mod fetcher;
mod investor;
mod position;
mod strategy;
mod wallet;

use chrono::{DateTime, Duration, Utc};
pub use coin::*;
pub use fetcher::*;
use ftx::rest::Rest;
pub use investor::*;
pub use position::*;
use std::env::var;
pub use strategy::*;
pub use wallet::*;

pub struct Trader {
    fetcher: Fetcher,
    strategy: Strategy,
    investor: Investor,
    rest: Rest,
}

impl Trader {
    pub fn new(coins: &[Coin], from: DateTime<Utc>, interval: Duration) -> Self {
        dotenv::dotenv().ok();
        let subaccount = Some(var("SUBACCOUNT").unwrap());
        let key = var("API_KEY").unwrap();
        let secret = var("API_SECRET").unwrap();
        let rest = Rest::new(key.clone(), secret.clone(), subaccount.clone());

        Trader {
            fetcher: Fetcher::new(coins, from, interval),
            strategy: Strategy::new(coins),
            investor: Investor::new(),
            rest,
        }
    }

    pub async fn run(mut self) {
        while let Some(prices) = self.fetcher.next(&self.rest).await {
            if let Some(position) = self.strategy.run(&prices) {
                self.investor.open(&prices, position).await;
                self.investor.close(&prices).await;
            }
        }

        log::info!(
            "TOTAL PROFIT: \t{:.2}",
            self.investor.total_realized_profit()
        );
        let (wins, losses) = self.investor.wins_losses();
        log::info!(
            "WIN/LOSS:     \t{}/{} ({:.2}%)",
            wins,
            losses,
            wins as f32 / (wins + losses) as f32 * 100.0
        );
    }
}
