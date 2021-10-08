use std::time::SystemTime;
use crate::hasher::hash;

// struct Key(pub[u8; len]);
struct Key(u8);

impl Key {
    pub fn new() -> Self {
        hash();
        todo!("hashing using realtime")
    }

}
struct Node {
    pub ip: String,
    pub port: u8,
    pub id: Key,
}