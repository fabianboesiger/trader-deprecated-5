#![allow(dead_code)]

mod change;
mod corr;
mod cov;
mod cum;
mod sma;
mod stdev;
//mod norm;

// Use SMA to compute moving averages.
pub use change::Change;
pub use corr::Corr;
pub use cov::Cov;
pub use cum::Cum;
pub use sma::Sma as Ma;
pub use stdev::Stdev;
//pub use norm::Norm;
