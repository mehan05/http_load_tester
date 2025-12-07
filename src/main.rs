pub mod state;
use std::time::Duration;

use clap::Parser;
use state::*;

pub mod control;
use control::*;

pub mod utils;
use utils::*;

#[tokio::main]
async fn main() {
    let mut metrics = Metrics{
        total_requests:0,
        RPS:0.0,
        error_rate:0.0,
        min_latency:Duration::ZERO,
        max_latency:Duration::ZERO,
        p95_latency:Duration::ZERO
    };


    let cli = Cli::parse();

    send_async_req(cli, &mut metrics).await;

    println!("{:?}",metrics);

}
