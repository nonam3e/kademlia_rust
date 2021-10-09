use crate::hasher;
use crate::settings::{KEY_LEN, N_MODE, NetworkMode};
use super::utils;
use std::fmt;
use std::fmt::Formatter;
use hex;

pub struct Key(pub[u8; KEY_LEN]);


impl Key {
    pub async fn new() -> Self {
        Self(hasher::hash().await)
    }
}
impl fmt::Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}",hex::encode(self.0))
    }
}
pub struct Node {
    pub ip: String,
    pub port: u16,
    pub id: Key,
}

impl Node {
    pub async fn new() -> Self {
        let ip = match N_MODE{
            NetworkMode::Global => utils::get_public_ip().await.unwrap(),
            NetworkMode::Local => utils::get_local_ip().await.unwrap(),
            _ => panic!("set NetworkMode"),
        };
        let port: u16 = 12121;
        let id = Key::new().await;
        Self{ip,port,id}
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}:{}\n{}",self.ip,self.port,self.id.to_string())
    }
}