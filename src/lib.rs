pub mod node {
    pub struct Node {
        id: u32,
        name: String,
        x: i32,
        y: i32,
        h: i32,
        adj_nodes: Vec<u32>,
    }
    
    impl Node {
        pub fn new(id: u32, name: &str, x: i32, y: i32, h: i32, adj_nodes: Vec<u32>) -> Node {
            Node{
                id,
                name: String::from(name),
                x,
                y,
                h,
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

pub struct Tree<'a>{
   pub root: &'a Node,
}

impl<'a> Tree<'a> {
    pub fn new(root: &'a Node) -> Tree {
       Tree {
           root,
       }
    }
}

use quick_xml::{Reader, events::Event};
use quick_xml::events::attributes::*;
use std::str;
pub fn parse_map(filename: &str) {
    let mut reader = Reader::from_file(filename).unwrap(); 
   
    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();
    reader.trim_text(true);
    
    loop {
        match reader.read_event(&mut buf){
            Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"map" => println!("attributes values: {:?}",
                                            e.attributes().map(|a|  { a.unwrap().value.into_owned() }).collect::<Vec<_>>()),
                        b"city" => count += 1,
                        _ => (),
                    }
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    println!("{:?} count: {}", txt, count); 
    // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
    buf.clear();
    }
} 
