use crate::Number;

#[derive(Copy, Clone)]
pub struct Cum {
    sum: Number,
}

impl Cum {
    pub fn new() -> Self {
        Self { sum: 0.0 }
    }

    pub fn run(&mut self, input: Number) -> Number {
        self.sum += input;
        self.sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut cum = Cum::new();

        assert_eq!(cum.run(10.0), 10.0);
        assert_eq!(cum.run(40.0), 50.0);
        assert_eq!(cum.run(-100.0), -50.0);
    }
}
