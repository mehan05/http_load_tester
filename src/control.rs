use std::clone;
use std::time::{Duration, Instant};

use clap::Parser;
use indicatif::ProgressBar;
use reqwest::Client;
use tokio::{sync::mpsc, task::*};
use tokio::{task, time};

use crate::state::{Cli, HttpRequestMethods, Metrics};
use crate::utils::{ header_mapper, header_spliter};

#[derive(Debug,Clone, Copy)]
pub struct MetricsCalculateValues{
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

    let progressbar = ProgressBar::new(duration.as_secs() );
    let start_time = Instant::now();

    

    let mut handlers: Vec<JoinHandle<( Vec<std::time::Duration>)>> = Vec::with_capacity(concurrency as usize);
    let (tx,mut rx) = mpsc::channel::<MetricsCalculateValues>(concurrency);
     let headers = header_mapper(&cli.header);
     for worked_ids in 0..concurrency{
         let url = url.clone();
         let client = client.clone();
         let tx = tx.clone();
         let headers = headers.clone();
         let method = http_method;
         let pb = progressbar.clone();
         let handler = task::spawn(async move {
             
             let mut metrics = MetricsCalculateValues{
                                errors:0,
                                start_time:Instant::now(),
                                time_taken:std::time::Duration::ZERO,
                            };
            let mut latencies:Vec<Duration> = vec![];

                for _ in 0..req_per_worker{
                    let req_start_time = Instant::now();
                    let deadline = start_time + duration;
                       if Instant::now()>deadline{
                                break;
                            }

                    let remaining_time = deadline.saturating_duration_since(req_start_time);

                            let mut req = match method{
                                      HttpRequestMethods::GET=>
                                      {
                                       client.get(&url)
                                      },
                                    HttpRequestMethods::POST=>client.post(&url),
                                    HttpRequestMethods::PUT=>client.put(&url),
                                    HttpRequestMethods::PATCH=>client.patch(&url),
                                    HttpRequestMethods::DELETE=>client.delete(&url),
                            };

                            if !headers.is_empty(){
                               req =  req.headers(headers.clone());
                            }
                            let resp = time::timeout(remaining_time,  req.send()).await.ok().and_then(|r|r.ok());

                            let time_taken  = req_start_time.elapsed();
                            latencies.push(time_taken);
                            

                            if let Some(resp) = resp{
                                let status = resp.status();

                                if (400..=499).contains(&status.as_u16()) || (500..=599).contains(&status.as_u16())
                                {
                                    metrics.errors=1;
                                    
                                }
                            }
                            
                            metrics.start_time = start_time;
                            metrics.time_taken = time_taken;

                            tx.send(metrics).await.unwrap();

                         

                }
            (latencies)
        });

        handlers.push(handler)
    }
drop(tx);

while let Some(rx)  = rx.recv().await{
    let rx = rx.clone();
   overall_metrics.total_requests= overall_metrics.total_requests.saturating_add(1);
    
    overall_metrics.total_errors = overall_metrics.total_errors.saturating_add(rx.errors);

   if overall_metrics.total_requests>0{
    overall_metrics.error_rate = (overall_metrics.total_errors*100).saturating_div(overall_metrics.total_requests) as f64;
    overall_metrics.error_rate
   }
   else{
    0.0
   };
   let elapsed_float_sec = start_time.elapsed().as_secs_f64().max(0.00001);
   
   let RPS = overall_metrics.total_requests as f64/elapsed_float_sec;
   overall_metrics.RPS = RPS;
   
   let elapsed_secs = start_time.elapsed().as_secs().min(duration.as_secs());
   progressbar.set_position(elapsed_secs);
   
   
   
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

overall_metrics.min_latency = min_latency.unwrap_or(Duration::ZERO);
overall_metrics.max_latency = max_latency.unwrap_or(Duration::ZERO);
overall_metrics.p95_latency = p95_latency;


}