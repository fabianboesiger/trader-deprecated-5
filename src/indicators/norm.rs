use super::{Ema, Stdev};
use crate::Number;

pub struct Norm {
    mean: Ema,
    stdev: Stdev,
    output: Option<Number>,
}

impl Norm {
    pub fn new(period: usize) -> Self {
        Self {
            mean: Ema::new(period),
            stdev: Stdev::new(period),
            output: None,
        }
    }

    pub fn run(&mut self, input: Number) {
        self.mean.run(input);
        self.stdev.run(input);
        if let (Some(stdev), Some(mean)) = (self.stdev.get(), self.mean.get()) {
            self.output = Some((input - mean) / stdev);
        } else {
            self.output = None;
        }
    }

    pub fn reset(&mut self) {
        self.mean.reset();
        self.stdev.reset();
        self.output = None;
    }

    pub fn get(&self) -> Option<Number> {
        self.output
    }

    pub fn get_mean(&self) -> Option<Number> {
        self.mean.get()
    }

    pub fn get_stdev(&self) -> Option<Number> {
        self.stdev.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {

    }
}
