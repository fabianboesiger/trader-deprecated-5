use super::Ma;
use crate::Number;

pub struct Stdev {
    var: Ma,
    mean: Ma,
}

impl Stdev {
    pub fn new(period: usize) -> Self {
        Self {
            var: Ma::new(period),
            mean: Ma::new(period),
        }
    }

    pub fn run(&mut self, input: Number) -> Number {
        let mean = self.mean.run(input);
        self.var.run((input - mean).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut stdev = Stdev::new(3);
        assert_eq!(stdev.run(100.0), 0.0);
        assert_eq!(stdev.run(100.0), 0.0);
        assert!(stdev.run(10.0) > 0.0);
    }
}
