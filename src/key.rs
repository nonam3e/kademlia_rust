use sha2::{Sha256, Digest};
use crate::utils::{get_current_time, get_local_ip, get_mac, get_public_ip, hash};
use crate::settings::{KEY_LEN, N_MODE, NetworkMode};

pub struct Key(pub[u8; KEY_LEN]);


impl Key {
    pub async fn new(port: u16) -> Self {
        let mut to_hash: String;
        match N_MODE {
            NetworkMode::Global => to_hash = get_public_ip().await.unwrap(),
            NetworkMode::Local => to_hash = get_local_ip().await.unwrap(),
            _=> panic!("set NetworkMode"),
        }
        to_hash = format!("{}{}{}{}", to_hash, get_mac().unwrap(),get_current_time().unwrap(),port);
        // println!("{}", to_hash);
        Self(hash(to_hash).await)
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",hex::encode(self.0))
    }
}

pub struct Distance(pub [u8; KEY_LEN]);

impl Distance {
    pub fn new(a: &Key, b: &Key) -> Distance {
        let mut res = [0;KEY_LEN];
        for i in 0..KEY_LEN {
            res[i] = a.0[i] ^ b.0[i];
        }
        Self(res)
    }
}
impl std::fmt::Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",hex::encode(self.0))
    }
}


