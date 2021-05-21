use crate::Coin;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub struct Position {
    pub open: bool,
    //pub open_time: DateTime<Utc>,
    //pub close_time: Option<DateTime<Utc>>,
    pub long: Coin,
    pub long_open_price: Decimal,
    pub long_close_price: Option<Decimal>,
    pub short: Coin,
    pub short_open_price: Decimal,
    pub short_close_price: Option<Decimal>,
}

impl Position {
    pub fn potential_profit(&self, long_close_price: Decimal, short_close_price: Decimal) -> Decimal {
        let fee = Decimal::new(5, 3);

        ((long_close_price - self.long_open_price) / self.long_open_price
                    + (self.short_open_price - short_close_price) / self.short_open_price)
                    - fee
    }

    pub fn profit(&self) -> Option<Decimal> {
        if let (Some(long_close_price), Some(short_close_price)) =
            (self.long_close_price, self.short_close_price)
        {
            Some(self.potential_profit(long_close_price, short_close_price))
        } else {
            None
        }
    }

    pub fn enter(&mut self) {

    }
}
