use super::Coin;
use chrono::{DateTime, Duration, DurationRound, Utc};
use ftx::rest::{Price, Rest};
use futures::future::join_all;
use std::collections::VecDeque;
use tokio::time::sleep;

pub struct Buf {
    coin: Coin,
    interval: Duration,
    curr: DateTime<Utc>,
    buf: VecDeque<Price>,
}

impl Buf {
    pub fn new(coin: Coin, from: DateTime<Utc>, interval: Duration) -> Self {
        let curr = from.duration_round(interval).unwrap();

        Buf {
            coin,
            interval,
            curr,
            buf: VecDeque::new(),
        }
    }

    pub async fn fetch(&mut self, rest: &Rest) {
        let sleep_duration = (self.curr - (Utc::now() - self.interval))
            .max(Duration::zero())
            .to_std()
            .unwrap();
        log::debug!("Sleeping for {:?}.", sleep_duration);
        sleep(sleep_duration).await;

        log::debug!("Fetching new batch of prices.");

        let prices = rest
            .get_historical_prices(
                &self.coin.to_string(),
                self.interval.num_seconds() as u32,
                Some(5000),
                Some(self.curr),
                Some(self.curr + self.interval * 5000),
            )
            .await
            .unwrap();

        for price in prices {
            if price.start_time < self.curr {
                continue;
            } else if price.start_time == self.curr {
                self.curr = price.start_time + self.interval;
                self.buf.push_back(price);
            } else {
                panic!("Missed time {:?} for coin {}", self.curr, self.coin);
            }
        }
    }

    pub async fn next(&mut self, rest: &Rest) -> Option<Price> {
        if let Some(price) = self.buf.pop_front() {
            Some(price)
        } else {
            loop {
                self.fetch(rest).await;

                if let Some(price) = self.buf.pop_front() {
                    return Some(price);
                }
            }
        }
    }
}

pub struct Fetcher<'a> {
    curr: DateTime<Utc>,
    coins: &'a [Coin],
    bufs: Vec<Buf>,
    interval: Duration,
}

impl<'a> Fetcher<'a> {
    pub fn new(coins: &'a [Coin], from: DateTime<Utc>, interval: Duration) -> Self {
        let curr = from.duration_round(interval).unwrap();

        let mut bufs = Vec::new();
        for &coin in coins {
            bufs.push(Buf::new(coin, from, interval))
        }

        Fetcher {
            curr,
            coins,
            bufs,
            interval,
        }
    }

    #[cfg(not(feature = "backtest"))]
    pub async fn next(&mut self, rest: &Rest) -> Option<Vec<Price>> {
        let mut vec: Vec<Price> = Vec::new();

        let mut futures = Vec::new();
        for buf in &mut self.bufs {
            futures.push(buf.next(rest));
        }

        let results = join_all(futures).await;
        for result in results {
            if let Some(next) = result {
                if let Some(first) = vec.first() {
                    assert_eq!(first.start_time, next.start_time);
                }
                vec.push(next);
            } else {
                return None;
            }
        }

        Some(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::var;

    #[tokio::test]
    async fn test_start_time() {
        dotenv::dotenv().ok();
        let subaccount = Some(var("SUBACCOUNT").unwrap());
        let key = var("API_KEY").unwrap();
        let secret = var("API_SECRET").unwrap();
        let rest = Rest::new(key.clone(), secret.clone(), subaccount.clone());

        let from = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        let mut buf = Buf::new(Coin::BTC, from, Duration::seconds(15));

        assert_eq!(buf.next(&rest).await.unwrap().start_time, from);
    }

    #[tokio::test]
    async fn test_multiple() {
        dotenv::dotenv().ok();
        let subaccount = Some(var("SUBACCOUNT").unwrap());
        let key = var("API_KEY").unwrap();
        let secret = var("API_SECRET").unwrap();
        let rest = Rest::new(key.clone(), secret.clone(), subaccount.clone());

        let from = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        let interval = Duration::seconds(15);
        let mut buf = Buf::new(Coin::BTC, from, Duration::seconds(15));

        let mut last = None;
        for i in 0..10000usize {
            let curr = buf.next(&rest).await.unwrap().start_time;
            if let Some(last) = last {
                assert_eq!(curr, last + interval, "Error at iteration {}", i);
            }
            last = Some(curr);
        }
    }

    #[tokio::test]
    async fn test_current() {
        dotenv::dotenv().ok();
        let subaccount = Some(var("SUBACCOUNT").unwrap());
        let key = var("API_KEY").unwrap();
        let secret = var("API_SECRET").unwrap();
        let rest = Rest::new(key.clone(), secret.clone(), subaccount.clone());

        let from = Utc::now();
        let interval = Duration::seconds(15);
        let mut buf = Buf::new(Coin::BTC, from, interval);

        assert!(buf.next(&rest).await.unwrap().start_time > from - interval);
    }

    #[tokio::test]
    async fn test_fetcher() {
        dotenv::dotenv().ok();
        let subaccount = Some(var("SUBACCOUNT").unwrap());
        let key = var("API_KEY").unwrap();
        let secret = var("API_SECRET").unwrap();
        let rest = Rest::new(key.clone(), secret.clone(), subaccount.clone());

        let from = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        let interval = Duration::seconds(15);
        let mut fetcher = Fetcher::new(&[Coin::BTC, Coin::ETH], from, interval);

        let mut last = None;
        for i in 0..10000usize {
            let curr = fetcher.next(&rest).await.unwrap()[0].start_time;
            if let Some(last) = last {
                assert_eq!(curr, last + interval, "Error at iteration {}", i);
            }
            last = Some(curr);
        }
    }
}
