use std::clone;
use std::time::{Duration, Instant};

use clap::Parser;
use reqwest::Client;
use tokio::{sync::mpsc, task::*};
use tokio::task;

use crate::state::{Cli, HttpRequestMethods, Metrics};
use crate::utils::{header_mapper, header_spliter};

#[derive(Debug,Clone, Copy)]
pub struct MetricsCalculateValues{
    total_requests:u64,
    errors:u64,
    start_time:Instant,
    time_taken:std::time::Duration,
}
pub async  fn send_async_req(cli:Cli,overall_metrics:&mut Metrics){

    let client = Client::new();
    let url = cli.url;
    let concurrency = cli.concurrency;
    let duration = cli.duration;
    let http_method = cli.method;
    let req_per_worker = 10;

    let mut handlers: Vec<JoinHandle<( Vec<std::time::Duration>)>> = Vec::with_capacity(concurrency as usize);
    let (tx,mut rx) = mpsc::channel::<MetricsCalculateValues>(concurrency);
     let headers = header_mapper(&cli.header);
     for worked_ids in 0..concurrency{
         let url = url.clone();
         let client = client.clone();
         let tx = tx.clone();
         let headers = headers.clone();
         let method = http_method;
         let handler = task::spawn(async move {
             
             let mut metrics = MetricsCalculateValues{
                                 total_requests:0,
                                errors:0,
                                start_time:Instant::now(),
                                time_taken:std::time::Duration::ZERO,
                            };
            let mut latencies:Vec<Duration> = vec![];

                for _ in 0..req_per_worker{
                    
                     

                       let start_time = Instant::now();

                            let mut req = match method{
                                      HttpRequestMethods::GET=>client.get(&url),
                                    HttpRequestMethods::POST=>client.post(&url),
                                    HttpRequestMethods::PUT=>client.put(&url),
                                    HttpRequestMethods::PATCH=>client.patch(&url),
                                    HttpRequestMethods::DELETE=>client.delete(&url),
                            };

                            if !headers.is_empty(){
                               req =  req.headers(headers.clone());
                            }
                            let resp = req.send().await.ok();

                            let time_taken  = start_time.elapsed();
                            latencies.push(time_taken);

                            if let Some(resp) = resp{
                                let status = resp.status();

                                if (400..=499).contains(&status.as_u16()) || (500..=599).contains(&status.as_u16())
                                {
                                    metrics.errors.saturating_add(1);
                                }
                            }
                            

                            metrics.total_requests = metrics.total_requests.saturating_add(1);
                            metrics.start_time = start_time;
                            metrics.time_taken = time_taken;

                            tx.send(metrics).await.unwrap();

                            if Instant::now()>start_time+duration{
                                break;
                            }

                }
            (latencies)
        });

        handlers.push(handler)
    }
drop(tx);

let mut RPS = 0;

while let Some(rx)  = rx.recv().await{
    let rx = rx.clone();
   overall_metrics.total_requests= overall_metrics.total_requests.saturating_add(rx.total_requests);

   let error_rate = if overall_metrics.total_requests>0{
    overall_metrics.error_rate = (rx.errors*100).saturating_div(overall_metrics.total_requests) as f64;
    overall_metrics.error_rate
   }
   else{
    0.0
   };

   RPS = overall_metrics.total_requests.saturating_div(rx.time_taken.as_secs() as u64);
   overall_metrics.RPS = RPS;
   


}
 let mut all_latencies:Vec<Duration> = Vec::new();

for handle in handlers{
    let (latencies)= handle.await.unwrap();
    all_latencies.extend(latencies);
}

all_latencies.sort();
let p95_latency =all_latencies [(all_latencies.len() as f64 * 0.95) as usize-1];

let min_latency = all_latencies.first().copied();

let max_latency = all_latencies.last().copied();

    
}