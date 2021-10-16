mod node;
mod hasher;
mod settings;
mod utils;
#[tokio::main]
async fn main() {
    let first_node = node::Node::new(None).await;
    let second_node = node::Node::new(Some(10002)).await;
    println!("{}",first_node.to_string());
    println!("{}",second_node.to_string());
    let distance = utils::Distance::new(&first_node.id,&second_node.id);
    print!("{}",distance);
}
