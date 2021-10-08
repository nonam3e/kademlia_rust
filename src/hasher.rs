//use tokio::net::UdpSocket;
use std::time::SystemTime;
use sha2::{Sha256, Digest};
use public_ip;
use mac_address::get_mac_address;

// async fn get_local_ip() -> Option<String> {
//     let socket = match UdpSocket::bind("0.0.0.0:0").await {
//         Ok(s) => s,
//         Err(_) => return None,
//     };
//
//     match socket.connect("8.8.8.8:80").await {
//         Ok(()) => (),
//         Err(_) => return None,
//     };
//
//     match socket.local_addr() {
//         Ok(addr) => return Some(addr.ip().to_string()),
//         Err(_) => return None,
//     };
// }

#[tokio::main]

pub async fn hash() -> String {
    let b= match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    let mut to_hash: String = b.to_string().clone();
    if let Some(ip) = public_ip::addr().await {
        to_hash = format!("{}{}", to_hash, ip.to_string());
    }
    // let ip = public_ip::addr().await;
    // let ip = String::from(ip.unwrap());
    let mac = match get_mac_address() {
        Ok(Some(ma)) => ma.to_string(),
        Ok(None) => String::from(""),
        Err(_e) => String::from(""),
    };
    to_hash = format!("{}{}", to_hash, mac);
    println!("{}", to_hash);
    let mut hasher = Sha256::new();
    hasher.update(to_hash);
    let hex = hasher.finalize();
    print!("{:x}",hex);
    format!("{:?}",hex)
}
