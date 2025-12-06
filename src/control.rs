use std::time::Instant;

use clap::Parser;
use reqwest::Client;
use tokio::{sync::mpsc, task::*};
use tokio::task;

use crate::state::{Cli, HttpRequestMethods, Metrics};
use crate::utils::header_spliter;

pub struct MetricsCalculateValues{
    total_requests:u64,
    errors:u64,
    start_time:Instant,
    time_taken:std::time::Duration,
}
pub async  fn send_async_req(cli:Cli,metrics:&mut Metrics){

    let client = Client::new();
    let url = cli.url;
    let concurrency = cli.concurrency;
    let duration = cli.duration;
    let http_method = cli.method;
    let req_per_worker = 10;

    let mut handlers: Vec<JoinHandle<()>> = Vec::new();
    let (tx,rx) = mpsc::channel::<MetricsCalculateValues>(concurrency);

    for worked_ids in 0..concurrency{
        let url = url.clone();
        let client = client.clone();
        let tx = tx.clone();

        let handlers = task::spawn(async move {

                for _ in 0..req_per_worker{
                    
                      let mut metrics = MetricsCalculateValues{
                         total_requests:0,
                        errors:0,
                        start_time:Instant::now(),
                        time_taken:std::time::Duration::ZERO,
                    };

                    match cli.method{
                        HttpRequestMethods::GET=>{
                            let start_time = Instant::now();
                            let mut  headers:Vec<(String,String)> = Vec::new();
                            if cli.header.clone().len()>2{
                                println!("Only two headers are allowed");
                                return;
                            }
                            
                            if cli.header.clone().len() > 0 {
                                for h in 0..cli.header.clone().len(){
                                    let (key,value) = header_spliter(cli.header[h].clone());
                                    headers.push((key,value));
                                }
                            }

                            let _req = client.get(&url.to_string())
                            .headers()
                            .send()
                            .await
                            .ok();
                            let time_taken = start_time.elapsed();
                  
                            if let Some(resp) = _req{
                                    let status = resp.status();
                                    if status.as_u16() >= 400 && status.as_u16() <= 499 || status.as_u16() >= 500 && status.as_u16() <= 599 {
                                        metrics.errors.saturating_add(1);
                                    }
                            }
                            metrics.total_requests.saturating_add(1);
                            metrics.start_time = start_time;
                            metrics.time_taken = time_taken;

                            tx.send(metrics).await.unwrap();
                        },
                        HttpRequestMethods::POST=>{

                        },
                        HttpRequestMethods::PUT=>{

                        },
                        HttpRequestMethods::PATCH=>{

                        },
                        HttpRequestMethods::DELETE=>{

                        },
                    }

                }
            

        });
    }


    
}