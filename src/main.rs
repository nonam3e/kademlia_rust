mod node;
mod key;
mod settings;
mod utils;
mod kbucket;
mod rpc;

#[tokio::main]
async fn main() {
    let first_node = node::Node::new(None).await;
    let second_node = node::Node::new(Some(10002)).await;
    println!("{}",first_node);
    println!("{}",second_node);
    let distance = key::Distance::new(&first_node.id,&second_node.id);
    print!("{}",distance);
}
