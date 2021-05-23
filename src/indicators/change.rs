use crate::Number;

#[derive(Copy, Clone)]
pub struct Change {
    last: Option<Number>,
}

impl Change {
    pub fn new() -> Self {
        Self { last: None }
    }

    pub fn run(&mut self, input: Number) -> Number {
        let output = if let Some(last) = self.last {
            (input - last) / last
        } else {
            0.0
        };
        self.last = Some(input);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut change = Change::new();

        assert_eq!(change.run(10.0), 0.0);
        assert_eq!(change.run(40.0), 30.0);
        assert_eq!(change.run(-20.0), -30.0);
    }
}
