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
    let start_time = Instant::now();
    let res = client.get(uri.clone()).await.unwrap();
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    let status = res.status().clone(); 
    let body_bytes = hyper::body::to_bytes(res.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    println!(
        "Request Number [{}]\nRequest took: [{:.2?}]\nStatus: [{}]\nContents: n[\n{}\n]",
        id,
        elapsed_time,
        status,
        body_str
    );

    Ok(())
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
            println!("#[Amount] how many requests you want to send");
        },
        2 => {
            let _ = send_req(&args[1].to_string(),&0).await.unwrap();
        },
        3 => {
            if let Ok(num) = args[2].parse::<u32>(){
                let adr =  args[1].clone();
                let mut handles = vec![];
                for i in 0..num{
                        let adr_clone = adr.clone();
                        let handle = tokio::spawn(async move {
                          let _ = send_req(&adr_clone.to_string(),&i).await.unwrap(); 
                        });
                        handles.push(handle);
                    }
                for handle in handles{
                        handle.await.expect("Failed to join thread"); 
                    }
                }
                else{
                println!("Error: Paramater [{}] is non-numeric",args[2]);
            }
        },
        _ => {},
    };
}

