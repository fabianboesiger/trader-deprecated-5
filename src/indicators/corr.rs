use super::{Cov, Stdev};
use crate::Number;

pub struct Corr {
    cov: Cov,
    stdev_x: Stdev,
    stdev_y: Stdev,
}

impl Corr {
    pub fn new(period: usize) -> Self {
        assert!(period >= 1);

        Self {
            cov: Cov::new(period),
            stdev_x: Stdev::new(period),
            stdev_y: Stdev::new(period),
        }
    }

    pub fn run(&mut self, x: Number, y: Number) -> Number {
        let d = self.stdev_x.run(x) * self.stdev_y.run(y);
        if d == 0.0 {
            0.0
        } else {
            self.cov.run(x, y) / d
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;

    #[test]
    fn correlated() {
        let mut corr = Corr::new(3);

        for i in 0..100 {
            assert!(corr.run(i as f32, i as f32) >= 0.0);
        }
    }

    #[test]
    fn not_correlated() {
        let mut corr = Corr::new(3);

        for i in 0..100 {
            assert!(corr.run(i as f32, -i as f32) <= 0.0);
        }
    }

    #[test]
    fn range() {
        let mut corr = Corr::new(3);

        for _ in 0..1000 {
            let corr = corr.run(random(), random());
            assert!(-1.0 <= corr && corr <= 1.0);
        }
    }
}
