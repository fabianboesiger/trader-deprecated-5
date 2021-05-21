use super::Ma;

use crate::Number;

pub struct Cov {
    x_avg: Ma,
    y_avg: Ma,
    cov: Ma,
}

impl Cov {
    pub fn new(period: usize) -> Self {
        Self {
            x_avg: Ma::new(period),
            y_avg: Ma::new(period),
            cov: Ma::new(period),
        }
    }

    pub fn run(&mut self, x: Number, y: Number) -> Number {
        self.cov
            .run((x - self.x_avg.run(x)) * (y - self.y_avg.run(y)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correlated() {
        let mut cov = Cov::new(3);

        for i in 0..100 {
            assert!(cov.run(i as f32, i as f32) >= 0.0);
        }
    }

    #[test]
    fn not_correlated() {
        let mut cov = Cov::new(3);

        for i in 0..100 {
            assert!(cov.run(i as f32, -i as f32) <= 0.0);
        }
    }
}
