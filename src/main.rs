pub mod state;
use clap::Parser;
use state::*;

pub mod control;
use control::*;

pub mod utils;
use utils::*;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let metrics = Metrics{
        total_requests:0,
        RPS:0,
        error_rate:0.0,
        min_latency:0,
        max_latency:0,
        p95_latency:0
    };


    let cli = Cli::parse();
}
