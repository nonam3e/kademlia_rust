use crate::hasher;
use crate::settings::{KEY_LEN, N_MODE, NetworkMode};
use super::utils;

pub struct Key(pub[u8; KEY_LEN]);


impl Key {
    pub fn new() -> Self {
        Self(hasher::hash())
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
        let id = Key::new();
        Self{ip,port,id}
    }
}