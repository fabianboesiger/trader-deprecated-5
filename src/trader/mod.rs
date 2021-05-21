mod coin;
mod fetcher;

use chrono::{DateTime, Duration, Utc};
use coin::Coin;
use fetcher::Fetcher;

pub struct Trader<'a> {
    fetcher: Fetcher<'a>,
}

impl<'a> Trader<'a> {
    pub fn new(coins: &'a [Coin], from: DateTime<Utc>, interval: Duration) -> Self {
        Trader {
            fetcher: Fetcher::new(coins, from, interval),
        }
    }
}
