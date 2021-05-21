use crate::Number;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Sma {
    data: VecDeque<Number>,
    sum: Number,
    period: usize,
}

impl Sma {
    pub fn new(period: usize) -> Self {
        debug_assert!(period >= 1);

        Self {
            data: VecDeque::new(),
            sum: 0.0,
            period,
        }
    }

    pub fn run(&mut self, input: Number) -> Number {
        self.data.push_back(input);
        self.sum += input;
        if self.data.len() > self.period as usize {
            let sub = self.data.pop_front().unwrap();
            self.sum -= sub;
        }
        self.sum / self.data.len() as Number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut sma = Sma::new(3);

        assert_eq!(sma.run(2.0), 2.0);
        assert_eq!(sma.run(4.0), 3.0);
        assert_eq!(sma.run(6.0), 4.0);
        assert_eq!(sma.run(8.0), 6.0);
    }
}
