mod node;
mod hasher;
pub mod settings;
mod utils;
use hex;
#[tokio::main]
async fn main() {
    let first_node = node::Node::new().await;
    print!("{}",first_node.to_string());
}
