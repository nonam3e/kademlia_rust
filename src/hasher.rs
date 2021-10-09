use sha2::{Sha256, Digest};
use crate::utils::{get_current_time, get_local_ip, get_mac, get_public_ip};
use super::settings::{N_MODE,KEY_LEN,NetworkMode,ALPHA};


pub async fn hash() -> [u8;KEY_LEN] {
    let mut to_hash: String;
    match N_MODE {
        NetworkMode::Global => to_hash = get_public_ip().await.unwrap(),
        NetworkMode::Local => to_hash = get_local_ip().await.unwrap(),
        _=> panic!("set NetworkMode"),
    }

    to_hash = format!("{}{}{}", to_hash, get_mac().unwrap(),get_current_time().unwrap());
    // println!("{}", to_hash);
    let mut hasher = Sha256::new();
    hasher.update(to_hash);
    let hex = hasher.finalize();
    let mut result = [0;KEY_LEN];
    for i in 0..KEY_LEN {
        result[i]=hex[i];
    }
    result
}
