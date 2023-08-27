use tokio::task;
use std::env;
use hyper::{Client, Uri};
use std::time::{Instant, Duration};
async fn send_req(adr: &str, id:&u32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let uri = match adr.parse::<Uri>() {
        Ok(uri) => uri,
        Err(error) => {
            println!("URI parsing error: {}", error);
            return Ok(());
        }
    };
    let res = client.get(uri.clone()).await.unwrap();
    let status = res.status().clone(); 
    let body_bytes = hyper::body::to_bytes(res.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec())?;

    println!(
        "Request Number [{}]\nStatus: [{}]\nContents: n[\n{}\n]",
        id,
        status,
        body_str
    );

    Ok(())
}


fn calculate_average_duration(durations: &Vec<Duration>) -> Duration {
    let num_durations = durations.len() as u32;

    if num_durations > 0 {
        *durations.last().unwrap() / num_durations
    } else {
        Duration::from_secs(0)
    }
}


#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        1 => {
            // Display help menu
            println!("#Welcome to CrabPing! A endpoint tester made in rust!\n============================");
            println!("#CrabPing [HttpReq] [Amount]\n============================");
            println!("#Running just CrabPing shows the help menu that you're using");
            println!("#[HttpReq] this is the endpoint you want to hit");
            println!("#[Amount] how many requests you want to send, max is 1000");
        },
        2 => {
            let start = Instant::now();
            let _ = send_req(&args[1].to_string(),&0).await.unwrap();
            let end = Instant::now();
            println!("Request took: [{:.2?}]",end - start);
        },
        3 => {
            if let Ok(num) = args[2].parse::<u32>(){
                let adr =  args[1].clone();
                let mut handles = vec![];
                let mut times: Vec<Duration> = vec![];
                for i in 0..num{
                        let adr_clone = adr.clone();
                        let handle = tokio::spawn(async move {
                          let _ = send_req(&adr_clone.to_string(),&i).await.unwrap(); 
                        });
                        handles.push(handle);
                    }
                for handle in handles{
                        let start = Instant::now();
                        handle.await.expect("Failed to join thread");
                        let end = Instant::now();
                        let dur = end - start;
                        times.push(dur);
                    }
                :x
                println!("Speed Report:\n=============\nAverage:[{:.2?}]",
                    calculate_average_duration(&times),
                    );
                }
                else{
                println!("Error: Paramater [{}] is non-numeric",args[2]);
            }
        },
        _ => {},
    };
}

