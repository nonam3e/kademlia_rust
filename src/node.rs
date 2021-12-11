use crate::settings::{KEY_LEN, N_MODE, NetworkMode};
// use hex;

pub struct Node {
    pub ip: String,
    pub port: u16,
    pub id: crate::key::Key,
}

impl Node {
    pub async fn new(port: Option <u16>) -> Self {
        let ip = match N_MODE{
            NetworkMode::Global => crate::utils::get_public_ip().await.unwrap(),
            NetworkMode::Local => crate::utils::get_local_ip().await.unwrap(),
            _ => panic!("set NetworkMode"),
        };
        let port= port.unwrap_or(12121);
        let id = crate::key::Key::new(port).await;
        Self{ip,port,id}
    }
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}:{}\n{}",self.ip,self.port,self.id.to_string())
    }
}