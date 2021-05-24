use super::{Candles, Coin};
use chrono::{DateTime, Utc};
use rust_decimal::prelude::*;

pub struct Position {
    pub open_time: Option<DateTime<Utc>>,
    pub close_time: Option<DateTime<Utc>>,
    pub long: Coin,
    pub long_open_price: Option<Decimal>,
    pub long_close_price: Option<Decimal>,
    pub long_quantity: Option<Decimal>,
    pub short: Coin,
    pub short_open_price: Option<Decimal>,
    pub short_close_price: Option<Decimal>,
    pub short_quantity: Option<Decimal>,
    pub take_profit: Decimal,
    pub stop_loss: Decimal,
}

impl Position {
    pub fn new(long: Coin, short: Coin, diff: Decimal) -> Self {
        Position {
            open_time: None,
            close_time: None,
            long,
            long_open_price: None,
            long_close_price: None,
            long_quantity: None,
            short,
            short_open_price: None,
            short_close_price: None,
            short_quantity: None,
            take_profit: diff,
            stop_loss: -diff,
        }
    }

    pub fn is_open(&self) -> bool {
        self.open_time.is_some()
    }

    pub fn is_closed(&self) -> bool {
        self.close_time.is_some()
    }

    pub fn open(&mut self, prices: &Candles, amount: Decimal) {
        assert!(!self.is_open());
        assert!(!self.is_closed());

        let half = amount / Decimal::new(2, 0);
        let long_price = prices[self.long as usize].close;
        let short_price = prices[self.short as usize].close;

        self.long_open_price = Some(long_price);
        self.short_open_price = Some(short_price);
        self.long_quantity = Some(half);
        self.short_quantity = Some(half);

        assert_eq!(
            prices[self.long as usize].time,
            prices[self.short as usize].time
        );
        let time = prices[self.long as usize].time;

        self.open_time = Some(time);

        log::info!(
            "OPEN  \t{}/{} \t= {:.4} \t@ {}",
            self.long,
            self.short,
            long_price / short_price,
            time
        );
    }

    pub fn check_close(&mut self, prices: &Candles) -> Option<Decimal> {
        assert!(self.is_open());
        assert!(!self.is_closed());

        if self.potential_profit_prices(prices) > self.take_profit {
            // Take profit.
            return Some(self.close(prices));
        } else if self.potential_profit_prices(prices) < self.stop_loss {
            // Stop loss.
            return Some(self.close(prices));
        }

        None
    }

    pub fn close(&mut self, prices: &Candles) -> Decimal {
        let long_price = prices[self.long as usize].close;
        let short_price = prices[self.short as usize].close;

        self.long_close_price = Some(long_price);
        self.short_close_price = Some(short_price);

        assert_eq!(
            prices[self.long as usize].time,
            prices[self.short as usize].time
        );
        let time = prices[self.long as usize].time;

        self.close_time = Some(time);

        log::info!(
            "CLOSE \t{}/{} \t= {:.4} \t@ {} \t PROFIT = {:.2}%",
            self.long,
            self.short,
            long_price / short_price,
            time,
            self.realized_profit()
        );

        self.realized_returns()
    }

    pub fn potential_profit_prices(&self, prices: &Candles) -> Decimal {
        self.potential_profit(
            prices[self.long as usize].close,
            prices[self.short as usize].close,
        )
    }

    pub fn potential_profit(
        &self,
        long_close_price: Decimal,
        short_close_price: Decimal,
    ) -> Decimal {
        assert!(self.is_open());
        assert!(!self.is_closed());

        (long_close_price - self.long_open_price.unwrap()) / self.long_open_price.unwrap()
            + (self.short_open_price.unwrap() - short_close_price) / self.short_open_price.unwrap()
    }

    pub fn realized_returns(&self) -> Decimal {
        assert!(self.is_open());
        assert!(self.is_closed());

        self.long_quantity.unwrap() / self.long_open_price.unwrap() * self.long_close_price.unwrap()
            + self.short_quantity.unwrap() / self.short_close_price.unwrap()
                * self.short_open_price.unwrap()
    }

    pub fn realized_profit(&self) -> Decimal {
        assert!(self.is_open());
        assert!(self.is_closed());

        self.long_quantity.unwrap() / self.long_open_price.unwrap() * self.long_close_price.unwrap()
            - self.long_quantity.unwrap()
            + self.short_quantity.unwrap() / self.short_close_price.unwrap()
                * self.short_open_price.unwrap()
            - self.short_quantity.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trader::fetcher::Candle;
    use chrono::TimeZone;

    #[tokio::test]
    async fn test_profit() {
        let time = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);

        let btc_open = Candle {
            close: Decimal::new(10, 0),
            //high: Decimal::zero(),
            //low: Decimal::zero(),
            volume: Decimal::zero(),
            //open: Decimal::zero(),
            time,
        };

        let eth_open = Candle {
            close: Decimal::new(20, 0),
            //high: Decimal::zero(),
            //low: Decimal::zero(),
            volume: Decimal::zero(),
            //open: Decimal::zero(),
            time,
        };

        let btc_close = Candle {
            close: Decimal::new(20, 0),
            //high: Decimal::zero(),
            //low: Decimal::zero(),
            volume: Decimal::zero(),
            //open: Decimal::zero(),
            time,
        };

        let eth_close = Candle {
            close: Decimal::new(10, 0),
            //high: Decimal::zero(),
            //low: Decimal::zero(),
            volume: Decimal::zero(),
            //open: Decimal::zero(),
            time,
        };

        let mut pos = Position::new(Coin::BTC, Coin::ETH, Decimal::zero());
        pos.open(&vec![btc_open, eth_open], Decimal::new(20, 0));
        assert_eq!(pos.close(&vec![btc_close, eth_close]), Decimal::new(40, 0));
    }
}
