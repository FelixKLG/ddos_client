use core::panic;

use reqwest::Client;

#[tokio::main]
async fn main() {
    let threadspawn = 10000;
    let target: &str;
    target = "";

    loop {
        for _j in 0..threadspawn {
            let http_client = Client::new();

            tokio::spawn(async move {
                let request = http_client.get(target).body("basic string").send().await;

                if let Ok(response) = request {
                    println!("Response: {}", response.status());
                } else {
                    println!("Request failed");
                }
            });
        }
    }
}
