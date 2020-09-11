pub mod node {
    pub struct Node {
        id: u32,
        name: String,
        x: i32,
        y: i32,
        adj_nodes: Vec<u32>,
    }
    
    impl Node {
        pub fn new(id: u32, name: &str, x: i32, y: i32, adj_nodes: Vec<u32>) -> Node {
            Node{
                id,
                name: String::from(name),
                x,
                y,
                adj_nodes,
            }
        }
    
        pub fn add_adj_node(&mut self, new_adj: u32) {
            self.adj_nodes.push(new_adj);
        }
    
        pub fn set_id(&mut self, new_id: u32) {
            self.id = new_id;
        }
    }
}

use node::Node;

pub struct Tree {
   pub root: Node,
}

impl Tree {
    pub fn new(root: Node) -> Tree {
       Tree {
           root,
       }
    }
}
