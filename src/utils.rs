use tokio::net::UdpSocket;
use std::time::SystemTime;
use public_ip;
use mac_address::get_mac_address;
use crate::node::Key;
use crate::settings::KEY_LEN;
use std::fmt;
use hex;


pub struct Distance(pub [u8; KEY_LEN]);

pub async fn get_local_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80").await {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}

pub async fn get_public_ip() -> Option<String>  {
    if let Some(ip) = public_ip::addr().await {
        return Some(ip.to_string());
    }
    return None;
}

pub fn get_mac() -> Option<String> {
    match get_mac_address() {
        Ok(Some(ma)) => return Some(ma.to_string()),
        Ok(None) => return None,
        Err(_e) => return None,
    }
}
pub fn get_current_time() -> Option<String> {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => Some(n.as_secs().to_string()),
        Err(_) => None,
    }
}

impl Distance {
    pub fn new(a: &Key, b: &Key) -> Distance {
        let mut res = [0;KEY_LEN];
        for i in 0..KEY_LEN {
            res[i] = a.0[i] ^ b.0[i];
        }
        Self(res)
    }
}
impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",hex::encode(self.0))
    }
}