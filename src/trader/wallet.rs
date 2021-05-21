use rust_decimal::prelude::*;

pub struct Wallet {
    balance: Decimal,
    parts: usize,
    borrowed: usize,
}

impl Wallet {
    pub fn new(balance: Decimal, parts: usize) -> Self {
        Wallet {
            balance,
            parts,
            borrowed: 0,
        }
    }

    pub fn borrow(&mut self) -> Option<Decimal> {
        if self.borrowed < self.parts {
            let loan = self.balance / Decimal::from_usize(self.parts - self.borrowed).unwrap();
            self.balance -= loan;
            self.borrowed += 1;
            Some(loan)
        } else {
            None
        }
    }

    pub fn put(&mut self, amount: Decimal) {
        self.balance += amount;
        self.borrowed -= 1;
    }

    pub fn update(&mut self, amount: Decimal) {
        self.balance = amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow() {
        let mut wallet = Wallet::new(Decimal::new(100, 0), 4);
        assert_eq!(wallet.borrow(), Some(Decimal::new(25, 0)));
        assert_eq!(wallet.borrow(), Some(Decimal::new(25, 0)));
        assert_eq!(wallet.borrow(), Some(Decimal::new(25, 0)));
        assert_eq!(wallet.borrow(), Some(Decimal::new(25, 0)));
        assert_eq!(wallet.borrow(), None);
        wallet.put(Decimal::new(20, 0));
        assert_eq!(wallet.borrow(), Some(Decimal::new(20, 0)));
        wallet.put(Decimal::new(20, 0));
        wallet.put(Decimal::new(30, 0));
        assert_eq!(wallet.borrow(), Some(Decimal::new(25, 0)));
        assert_eq!(wallet.borrow(), Some(Decimal::new(25, 0)));
        assert_eq!(wallet.borrow(), None);
        wallet.put(Decimal::new(30, 0));
        wallet.put(Decimal::new(30, 0));
        wallet.put(Decimal::new(30, 0));
        wallet.put(Decimal::new(30, 0));
        assert_eq!(wallet.borrow(), Some(Decimal::new(30, 0)));
    }
}
