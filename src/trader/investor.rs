use crate::strategy::Action;

use super::wallet::Wallet;
use super::Position;

pub struct Investor {
    wallet: Wallet,
    positions: Vec<Position>,
}

impl Investor {
    fn action(&mut self, action: Action, data: Data) {
        match action {
            Action::Enter {
                long,
                short,
            } => {

            },
            Action::Exit {
                long,
                short,
            } => {

            }
        }
    }
}
