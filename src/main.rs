mod node;
mod hasher;
mod settings;
mod utils;
#[tokio::main]
async fn main() {
    let first_node = node::Node::new().await;
    print!("{}",first_node.to_string());
}
