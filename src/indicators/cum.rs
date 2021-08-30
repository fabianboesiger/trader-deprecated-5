use std::collections::VecDeque;

use crate::Number;

pub struct Cum {
    sum: Number,
    values: VecDeque<Number>,
    period: usize,
}

impl Cum {
    pub fn new(period: usize) -> Self {
        Self {
            sum: 0.0,
            values: VecDeque::new(),
            period
        }
    }

    pub fn run(&mut self, input: Number) -> Number {
        self.values.push_back(input);
        self.sum += input;
        if self.values.len() > self.period {
            self.sum -= self.values.pop_front().unwrap();
        }
        self.sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut cum = Cum::new(3);

        assert_eq!(cum.run(10.0), 10.0);
        assert_eq!(cum.run(40.0), 50.0);
        assert_eq!(cum.run(-100.0), -50.0);
        assert_eq!(cum.run(60.0), 0.0);
    }
}
