use super::{Action, Prices, Strategy};
use crate::{
    indicator::{Corr, Norm, Stdev, Cum},
    Coin, Number,
};
use chrono::Duration;
use num_traits::{FromPrimitive, Num};

struct Pair {
    corr: Corr,
    stdev: Stdev,
    last: Option<Number>,
    curr: Option<Number>,
}

impl Pair {
    pub fn new(period: usize) -> Self {
        Pair {
            corr: Corr::new(period),
            stdev: Stdev::new(period),
            last: None,
            curr: None,
        }
    }

    pub fn run(&mut self, x: Number, y: Number) {
        self.corr.run(x, y);
        self.stdev.run((x - y).abs());
        self.last = self.curr;
        self.curr = Some(x - y);
    }

    pub fn get_corr(&self) -> Option<Number> {
        self.corr.get()
    }

    pub fn enter(&self) -> bool {
        if let (Some(corr), Some(stdev), Some(curr)) = (self.corr.get(), self.stdev.get(), self.curr) {
            if corr > 0.95 {
                println!("{} {}", curr.abs(), 2.5 * stdev);
                if curr.abs() > 2.5 * stdev {
                    return true;
                }
            }
        }

        false
    }

    pub fn exit(&self) -> bool {
        if let Some(curr) = self.curr {
            if let Some(last) = self.last {
                if curr * last <= 0.0 {
                    return true;
                }
            }
        }

        false
    }
}

pub struct Custom {
    pairs: Vec<Vec<Pair>>,
    norm: Vec<(Number, Cum)>,
}

impl Custom {
    pub fn new(markets: &[Coin]) -> Self {
        let duration = Duration::days(60).num_minutes() as usize;

        let mut norm = Vec::new();
        for _ in 0..markets.len() {
            norm.push((f32::NAN, Cum::new()));
        }

        let mut pairs = Vec::new();
        for i in 0..markets.len() {
            let mut to = Vec::new();
            for _ in 0..i {
                to.push(Pair::new(duration))
            }
            pairs.push(to);
        }

        Custom {
            norm,
            pairs,
        }
    }
}

impl Strategy for Custom {
    fn run(&mut self, prices: &Prices) -> Option<Action> {
        for i in 0..prices.len() {
            if !prices[i].is_nan() && !self.norm[i].0.is_nan() {
                let last = self.norm[i].0;
                self.norm[i].1.run(prices[i] - last);
            }
            self.norm[i].0 = prices[i];
        }

        for i in 0..prices.len() {
            for j in 0..i {
                //assert!(!self.norm[i].get().unwrap().is_nan() && !self.norm[j].get().unwrap().is_nan());
                let (norm_i, norm_j) = (self.norm[i].1.get(), self.norm[j].1.get());
                self.pairs[i][j].run(norm_i, norm_j);
                
                if self.pairs[i][j].enter() {
                    let position = if norm_i > norm_j {
                        Action::Enter {
                            long: Coin::from_usize(j).unwrap(),
                            short: Coin::from_usize(i).unwrap(),
                        }
                    } else {
                        Action::Enter {
                            long: Coin::from_usize(i).unwrap(),
                            short: Coin::from_usize(j).unwrap(),
                        }
                    };
                    return Some(position);
                } else if self.pairs[i][j].exit() {
                    let position = if norm_i < norm_j {
                        Action::Exit {
                            long: Coin::from_usize(j).unwrap(),
                            short: Coin::from_usize(i).unwrap(),
                        }
                    } else {
                        Action::Exit {
                            long: Coin::from_usize(i).unwrap(),
                            short: Coin::from_usize(j).unwrap(),
                        }
                    };
                    return Some(position);
                }
            }
        }
        /*
        let mut pairs: Vec<(usize, usize, Number)> = self.pairs
            .iter()
            .enumerate()
            .map(|(i, p)| 
                p.iter().enumerate().map(move |(j, p)| (i, j, p.get_corr()))
            )
            .flatten()
            .filter_map(|(i, j, c)| if let Some(c) = c {
                Some((i, j, c))
            } else {
                None
            })
            .collect();
        pairs.sort_by_key(|(_, _, c)| (c * 100000.0) as i32);
        println!("{:#?}", pairs);
        */
        None
    }
}
