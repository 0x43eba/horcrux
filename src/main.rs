mod algorithm;
mod transmit;
mod steg;
mod cli;
use std::env;

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();
    cli::handler::handle_input().await;
}