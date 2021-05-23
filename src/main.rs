mod indicators;
mod trader;
mod logger;

// Either f32 or f64.
type Number = f32;

use once_cell::sync::Lazy;
use chrono::{Utc, TimeZone, Duration};
use logger::Logger;
use trader::{Trader, Coin};

#[allow(dead_code)]
static LOGGER: Lazy<Logger> = Lazy::new(|| {
    Logger::new()
});

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let coins = Coin::all();
    let trader = Trader::new(
        &coins, 
        Utc.ymd(2021, 5, 1).and_hms(0, 0, 0),
        Duration::seconds(15),
    );
    trader.run().await
}
