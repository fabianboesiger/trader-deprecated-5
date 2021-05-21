mod custom;

use crate::coin::Coin;
pub use custom::Custom;

pub type Prices = Vec<f32>;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Enter { long: Coin, short: Coin },
    Exit { long: Coin, short: Coin },
}

pub trait Strategy {
    fn run(&mut self, prices: &Prices) -> Option<Action>;
}
