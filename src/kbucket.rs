use crate::node;
use crate::settings::{ K_PARAM };

use std::fmt;

pub struct Kbucket
{
    nodes:        Vec<node::Node>,
    last_update:  std::time::Instant,
    pending_node: Option < (node::Node, std::time::Instant) >,
}

impl Kbucket
{
    pub fn new() -> Self
    {
        return Kbucket {
            nodes: Vec::new(),
            last_update: std::time::Instant::now(),
            pending_node: None,
        }
    }

    fn update(&mut self, timeout: std::time::Duration)
    {
        if let Some((node, time)) = self.pending_node.take()
        {
            if time.elapsed() >= timeout
            {
                self.nodes.remove(0);
                self.nodes.push(node);
            }
            else
            {
                self.pending_node = Some((node, time));
            }
        }
    }

    fn add_node(&mut self, node: node::Node)
    {
        //
    }
}


impl fmt::Display for Kbucket
{
    fn fmt(&self, former :&mut fmt::Formatter <'_>) -> fmt::Result
    {
        let mut res :fmt::Result = write!(former, "kbucket:\n");

        let mut idx :usize = 1;
        for i in &self.nodes
        {
            if res == fmt::Result::Err(fmt::Error)
            {
                break;
            }
            res = write!(former, "node {}:\n{}\n", idx, i);
            idx += 1;
        }

        res
    }
}
