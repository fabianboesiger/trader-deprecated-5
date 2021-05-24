use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fmt;

/*
pub struct CoinVec<T>(Vec<Option<T>>);

impl<T> CoinVec<T> {
    pub fn new() -> Self {
        let vec = Vec::new();
        for _ in Coin::all() {
            vec.push(None);
        }
        CoinVec(vec)
    }

    pub fn set(&mut self, coin: Coin, data: T) {
        let entry = self.0.get_mut(coin as usize).unwrap();
        assert!(entry.is_none(), "Entry is already defined");
        *entry = Some(data);
    }

    pub fn get(&mut self, coin: Coin) -> &T {
        assert!(self.0.iter().fold(true, |a, b| a && b.is_some()), "Not all entries initialized.");
        let entry = self.0.get(coin as usize).unwrap();
        entry.as_ref().unwrap()
    }
}
*/

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq, Hash)]
pub enum Coin {
    BTC = 0,
    ETH = 1,
    DOGE = 2,
    BNB = 3,
    XRP = 4,
    ADA = 5,
    EOS = 6,
    BCH = 7,
    LINK = 8,
    SOL = 9,
    LTC = 10,
    DOT = 11,
    MATIC = 12,
    AAVE = 13,
    ETC = 14,
    FTT = 15,
    SUSHI = 16,
    ZEC = 17,
    XLM = 18,
    YFI = 19,
    TRX = 20,
}

impl Coin {
    pub fn all() -> Vec<Coin> {
        (0..=20).map(|i| Coin::from_usize(i).unwrap()).collect()
    }
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}-PERP", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        assert!(Coin::from_usize(Coin::all().len() - 1).is_some());
        assert!(Coin::from_usize(Coin::all().len()).is_none());
    }
}
