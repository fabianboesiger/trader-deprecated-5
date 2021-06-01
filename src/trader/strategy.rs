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
    stdev: Stdev,
    count: usize,
    out_diff: Number,
    out_enter: bool,
}

impl Pair {
    pub fn new(corr_period: usize) -> Self {
        Pair {
            corr: Corr::new(corr_period),
            stdev: Stdev::new(corr_period),
            count: corr_period,
            out_diff: 0.0,
            out_enter: false,
        }
    }

    pub fn run(&mut self, long: &Single, short: &Single) {
        let corr = self.corr.run(long.get_cum(), short.get_cum());
        self.out_diff = short.get_mov() - long.get_mov();
        let stdev = self.stdev.run(self.out_diff.abs());

        //LOGGER.pair(long.coin, short.coin).diff(self.out_diff);

        let max_diff = 0.1;
        let min_diff = 0.05;

        if self.count > 0 {
            // Backoff from trading for some time.
            self.count -= 1;
        } else {
            self.out_enter = corr > 0.95
                && self.out_diff > stdev * 2.5
                && max_diff >= self.out_diff
                && self.out_diff >= min_diff
                //&& long.get_price_adj_ma() > long.get_price_ma()
                //&& short.get_price_adj_ma() < short.get_price_ma();
        }
    }

    pub fn should_enter(&self) -> bool {
        self.out_enter
    }

    pub fn get_diff(&self) -> Number {
        self.out_diff
    }
}

pub struct Single {
    coin: Coin,
    change: Change,
    cum: Cum,
    mean: Ma,
    out_mov: Number,
    out_cum: Number,
    price: Number,
    price_ma: Ma,
    price_vol_ma: Ma,
    vol_ma: Ma,
    out_price_ma: Number,
    out_price_adj_ma: Number,
}

impl Single {
    pub fn new(coin: Coin, mov_period: usize) -> Self {
        Single {
            coin,
            change: Change::new(),
            cum: Cum::new(),
            mean: Ma::new(mov_period),
            out_mov: 0.0,
            out_cum: 0.0,
            price: 0.0,
            price_ma: Ma::new(100),
            price_vol_ma: Ma::new(100),
            vol_ma: Ma::new(100),
            out_price_ma: 0.0,
            out_price_adj_ma: 0.0,
        }
    }

    pub fn run(&mut self, candle: &Candle) {
        let price = candle.close.to_f32().unwrap();
        let volume = candle.volume.to_f32().unwrap();
        self.out_price_ma = self.price_ma.run(price);
        self.out_price_adj_ma = self.price_vol_ma.run(price * volume) / self.vol_ma.run(volume);

        self.out_cum = self.cum.run(self.change.run(price));
        let mean = self.mean.run(self.out_cum);
        self.out_mov = self.out_cum - mean;
    }

    pub fn get_price_ma(&self) -> Number {
        self.out_price_ma
    }

    pub fn get_price_adj_ma(&self) -> Number {
        self.out_price_adj_ma
    }

    pub fn get_cum(&self) -> Number {
        self.out_cum
    }

    pub fn get_mov(&self) -> Number {
        self.out_mov
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

        for (single, candle) in self.singles.iter_mut().zip(candles) {
            single.run(candle);
        }

        for (p, long) in self.pairs.iter_mut().zip(self.singles.iter()) {
            for (pair, short) in p.iter_mut().zip(self.singles.iter()) {
                pair.run(long, short);

                if pair.should_enter() {
                    return Some(Position::new(
                        long.coin,
                        short.coin,
                        Decimal::from_f32(pair.get_diff()).unwrap(),
                    ));
                }
            }
        }

        None
    }
}
