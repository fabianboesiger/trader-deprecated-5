mod indicators;
mod trader;

// Either f32 or f64.
type Number = f32;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
}
