mod node;
mod hasher;
pub mod settings;
mod utils;
#[tokio::main]
async fn main() {
    // print!("Hello world");
    let first_node = node::Node::new().await;
    print!("{}:{}",first_node.ip,first_node.port);
}
