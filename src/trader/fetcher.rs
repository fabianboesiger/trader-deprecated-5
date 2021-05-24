use super::Coin;
use chrono::{DateTime, Duration, DurationRound, Utc};
use ftx::rest::{Price, Rest};
use futures::future::join_all;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tokio::time::sleep;

pub type Candles = Vec<Candle>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Candle {
    pub close: Decimal,
    pub volume: Decimal,
    pub time: DateTime<Utc>,
}

impl From<Price> for Candle {
    fn from(price: Price) -> Self {
        Candle {
            close: price.close,
            volume: price.volume,
            time: price.start_time,
        }
    }
}

pub struct Buf {
    coin: Coin,
    interval: Duration,
    curr: DateTime<Utc>,
    buf: VecDeque<Candle>,
    last: Option<Candle>,
    #[allow(dead_code)]
    realtime: bool,
}

impl Buf {
    pub fn new(coin: Coin, from: DateTime<Utc>, interval: Duration) -> Self {
        let curr = from.duration_round(interval).unwrap();

        Buf {
            coin,
            interval,
            curr,
            buf: VecDeque::new(),
            last: None,
            realtime: false,
        }
    }

    pub async fn fetch(&mut self, rest: &Rest) {
        let sleep_duration = (self.curr - (Utc::now() - self.interval)).max(Duration::zero());
        log::debug!("Sleeping for {:?}.", sleep_duration);
        if sleep_duration > Duration::zero() {
            self.realtime = true;
        }
        sleep(sleep_duration.to_std().unwrap()).await;

        log::debug!("Fetching new batch of prices.");

        let start_time = self.curr - self.interval * 5;
        let end_time = self.curr + self.interval * 4995;

        let prices = rest
            .get_historical_prices(
                &self.coin.to_string(),
                self.interval.num_seconds() as u32,
                Some(5000),
                Some(start_time),
                Some(end_time),
            )
            .await
            .unwrap();

        if prices.len() == 0 {
            log::warn!("No data for coin {}.", self.coin);
            while self.curr < end_time {
                let last = self.last.unwrap();
                self.buf.push_back(Candle {
                    close: last.close,
                    volume: last.volume,
                    time: self.curr,
                });
                self.curr = self.curr + self.interval;
            }
        } else {
            for price in prices {
                if price.start_time < self.curr {
                    continue;
                } else if price.start_time == self.curr {
                    self.buf.push_back(price.into());
                    self.curr = price.start_time + self.interval;
                } else {
                    // Backfill until next data.
                    log::warn!(
                        "Expected time {:?}, got {:?} for coin {}",
                        self.curr,
                        price.start_time,
                        self.coin
                    );
                    while price.start_time > self.curr {
                        let last = self.last.unwrap();
                        self.buf.push_back(Candle {
                            close: last.close,
                            volume: last.volume,
                            time: self.curr,
                        });
                        self.curr = self.curr + self.interval;
                    }

                    // Insert next known data.
                    assert_eq!(price.start_time, self.curr);
                    self.buf.push_back(price.into());
                    self.curr = price.start_time + self.interval;
                }
                if let Some(&candle) = self.buf.back() {
                    self.last = Some(candle);
                }
            }
        }
    }

    pub async fn next(&mut self, rest: &Rest) -> Option<Candle> {
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

    pub fn is_realtime(&self) -> bool {
        self.realtime
    }
}

pub struct Fetcher {
    bufs: Vec<Buf>,
    #[cfg(feature = "backtest")]
    buf: Vec<Candles>,
    #[cfg(feature = "backtest")]
    fetched: bool,
}

impl Fetcher {
    pub fn new(coins: &[Coin], from: DateTime<Utc>, interval: Duration) -> Self {
        let curr = from.duration_round(interval).unwrap();

        let mut bufs = Vec::new();
        for &coin in coins {
            bufs.push(Buf::new(coin, from, interval))
        }

        Fetcher {
            bufs,
            #[cfg(feature = "backtest")]
            buf: Vec::new(),
            #[cfg(feature = "backtest")]
            fetched: false,
        }
    }

    async fn fetch(&mut self, rest: &Rest) -> Option<Candles> {
        let mut vec: Candles = Vec::new();

        let mut futures = Vec::new();
        for buf in &mut self.bufs {
            futures.push(buf.next(rest));
        }

        let results = join_all(futures).await;
        for result in results {
            if let Some(next) = result {
                if let Some(first) = vec.first() {
                    assert_eq!(first.time, next.time);
                }
                vec.push(next);
            } else {
                return None;
            }
        }

        Some(vec)
    }

    #[cfg(not(feature = "backtest"))]
    pub async fn next(&mut self, rest: &Rest) -> Option<Candles> {
        self.fetch(rest).await
    }

    // TODO: Make this non-blocking.
    #[cfg(feature = "backtest")]
    async fn fetch_and_cache(&mut self, rest: &Rest) {
        use std::time::SystemTime;
        use std::{
            fs::OpenOptions,
            io::{Read, Write},
        };

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("cache.bin")
            .unwrap();

        let metadata = file.metadata().unwrap();

        if metadata.len() > 0
        /*&& SystemTime::now()
        .duration_since(metadata.modified().unwrap())
        .unwrap()
        .as_secs()
        < 3600 * 8*/
        {
            // Load data from cache.
            log::info!("Loading backtest data from cache.");

            //let mut bin = Vec::new();
            //file.read_to_end(&mut bin).await.unwrap();
            //self.buf = bincode::deserialize(&bin[..]).unwrap();
            self.buf = serde_cbor::from_reader(file).unwrap();
        } else {
            // Fetch data and save to cache.
            log::info!("Loading backtest data from API.");

            while let Some(prices) = self.fetch(rest).await {
                self.buf.push(prices);
                if self.is_realtime() {
                    break;
                }
            }
            self.buf.reverse();

            log::info!("Saving backtest data to file.");

            //let bin = bincode::serialize(&self.buf).unwrap();
            //file.write_all(&bin[..]).await.unwrap();
            serde_cbor::to_writer(file, &self.buf).unwrap();
        }

        self.fetched = true;
        log::info!("Done loading backtest data!");
    }

    #[cfg(feature = "backtest")]
    pub async fn next(&mut self, rest: &Rest) -> Option<Candles> {
        if !self.fetched {
            self.fetch_and_cache(rest).await;
        }

        self.buf.pop()
    }

    fn is_realtime(&self) -> bool {
        self.bufs.iter().fold(false, |a, b| a || b.is_realtime())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
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

        assert_eq!(buf.next(&rest).await.unwrap().time, from);
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
            let curr = buf.next(&rest).await.unwrap().time;
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

        assert!(buf.next(&rest).await.unwrap().time > from - interval);
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
            let curr = fetcher.next(&rest).await.unwrap()[0].time;
            if let Some(last) = last {
                assert_eq!(curr, last + interval, "Error at iteration {}", i);
            }
            last = Some(curr);
        }
    }
}
