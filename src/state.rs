use std::time::Duration;

use clap::{Parser,ValueEnum};
use humantime::parse_duration;

#[derive(Debug,Clone,Copy,ValueEnum)]
pub enum HttpRequestMethods{
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

#[derive(Parser,Debug)]
#[command(name = "http_load_tester")]
pub struct Cli{

    #[arg(long)]
    pub url:String,

    #[arg(long, value_enum, default_value_t=HttpRequestMethods::GET)]
    pub method: HttpRequestMethods,

    #[arg(long, default_value_t=10)]
    pub concurrency:usize,

    #[arg(long , value_parser=parse_human_duration)]
    pub duration:Duration,

    #[arg(long)]
    pub data: Option<String>,

    #[arg(long)]
    pub header:Vec<String>,

    #[arg(long, value_parser=parse_human_duration,default_value = "5s")]
    pub timeout:Duration
}

#[derive(  Debug,Clone,Copy)]
pub struct Metrics{
    pub total_requests:u64,
    pub RPS:f64,
    pub error_rate:f64,
    pub total_errors:u64,
    pub min_latency:Duration,
    pub max_latency:Duration,
    pub p95_latency:Duration
}


pub fn parse_human_duration(duration:&str)->Result<Duration,String>{
    parse_duration(duration).map_err(|e| format!("Invalid time {}",e))
}