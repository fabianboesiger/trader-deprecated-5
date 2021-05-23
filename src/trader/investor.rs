use super::{Position, Wallet, Prices};
use rust_decimal::prelude::*;

pub struct Investor {
    wallet: Wallet,
    positions: Vec<Position>,
}

impl Investor {
    pub fn new() -> Self {
        Investor {
            wallet: Wallet::new(Decimal::new(100, 0), 1),
            positions: Vec::new(),
        }
    }

    pub async fn open(&mut self, prices: &Prices, mut position: Position) {
        if let Some(borrowed) = self.wallet.borrow() {
            position.open(prices, borrowed);
            self.positions.push(position);
        }
    }

    pub async fn close(&mut self, prices: &Prices) {
        for position in self.positions.iter_mut().filter(|p| p.is_open()) {
            if let Some(returned) = position.check_close(&prices) {
                self.wallet.put(returned);
            }
        }
    }
}
