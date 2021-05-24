use std::fmt::Pointer;

use super::{Candles, Position, Wallet};
use rust_decimal::prelude::*;

pub struct Investor {
    wallet: Wallet,
    positions: Vec<Position>,
}

impl Investor {
    pub fn new() -> Self {
        Investor {
            wallet: Wallet::new(Decimal::new(100, 0), 2),
            positions: Vec::new(),
        }
    }

    pub async fn open(&mut self, prices: &Candles, mut position: Position) {
        let already_invested = self
            .positions
            .iter()
            .filter(|p| p.long == position.long && p.short == position.short)
            .count()
            > 0;
        if !already_invested {
            if let Some(borrowed) = self.wallet.borrow() {
                position.open(prices, borrowed);
                self.positions.push(position);
            }
        }
    }

    pub async fn close(&mut self, prices: &Candles) {
        for position in self
            .positions
            .iter_mut()
            .filter(|p| p.is_open() && !p.is_closed())
        {
            if let Some(returned) = position.check_close(&prices) {
                self.wallet.put(returned);
            }
        }
    }

    pub fn total_realized_profit(&self) -> Decimal {
        self.positions
            .iter()
            .filter(|p| p.is_closed())
            .map(|p| p.realized_profit())
            .sum()
    }

    pub fn wins_losses(&self) -> (usize, usize) {
        let mut wins = 0;
        let mut losses = 0;
        for position in &self.positions {
            if position.is_closed() {
                if position.realized_profit() > Decimal::zero() {
                    wins += 1;
                } else {
                    losses += 1;
                }
            }
        }
        (wins, losses)
    }
}
