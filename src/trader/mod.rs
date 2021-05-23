mod coin;
mod fetcher;
mod position;
mod strategy;
mod investor;
mod wallet;

use chrono::{DateTime, Duration, Utc};
use ftx::rest::Rest;
use std::env::var;
pub use coin::*;
pub use fetcher::*;
pub use position::*;
pub use strategy::*;
pub use investor::*;
pub use wallet::*;

pub struct Trader<'a> {
    fetcher: Fetcher<'a>,
    strategy: Strategy,
    investor: Investor,
    rest: Rest,
}

impl<'a> Trader<'a> {
    pub fn new(coins: &'a [Coin], from: DateTime<Utc>, interval: Duration) -> Self {
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
    }
}
