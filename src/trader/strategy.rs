use super::{Candles, Candle, Coin, Position};
use crate::{
    indicators::{Change, Corr, Cum, Ma, Stdev},
    Number,
};
use num_traits::Num;
use rust_decimal::prelude::*;
use crate::LOGGER;

struct Pair {
    corr: Corr,
    count: usize,
    out_diff: Number,
}

impl Pair {
    pub fn new(corr_period: usize) -> Self {
        Pair {
            corr: Corr::new(corr_period),
            count: corr_period,
            out_diff: 0.0,
        }
    }

    pub fn corr(&mut self, long: &Single, short: &Single) -> Number {
        self.corr.run(long.get_cum(), short.get_cum())
    }
}

pub struct Single {
    coin: Coin,
    change: Change,
    cum: Cum,
    cum_out: Number,
}

impl Single {
    pub fn new(coin: Coin, cum_period: usize) -> Self {
        Single {
            coin,
            change: Change::new(),
            cum: Cum::new(cum_period),
            cum_out: 0.0,
        }
    }

    pub fn compute_cum(&mut self, candle: &Candle) {
        self.cum_out = self.cum.run(self.change.run(candle.close.to_f32().unwrap()))
    }

    pub fn get_cum(&self) -> Number {
        self.cum_out
    }
}

pub struct Strategy {
    singles: Vec<Single>,
    pairs: Vec<Vec<Pair>>,
}

impl Strategy {
    pub fn new(coins: &[Coin]) -> Self {
        let mut singles = Vec::new();
        for &coin in coins {
            singles.push(Single::new(coin, 60 * 60 * 24 / 15))
        }

        let mut pairs = Vec::new();
        for _ in coins {
            let mut p = Vec::new();
            for _ in coins {
                p.push(Pair::new(60 * 60 * 24 * 30 / 15))
            }
            pairs.push(p);
        }

        Strategy { singles, pairs }
    }

    pub fn run(&mut self, candles: &Candles) -> Option<Position> {
        //let prices_float: Vec<Number> = prices.iter().map(|d| d.close.to_f32().unwrap()).collect();

        for i in 0..self.singles.len() {
            self.singles[i].compute_cum(&candles[i]);
        }

        for i in 0..self.pairs.len() {
            for j in 0..self.pairs[i].len() {
                let corr = self.pairs[i][j].corr(&self.singles[i], &self.singles[j]);
                self.singles[i].add_price_corr(&self.singles[j], corr);
            }
            self.singles[i]
        }

        None
    }
}
