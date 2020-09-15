use le_rovine_perdute_rs::{Tree, node};

fn main() {
    println!("start");
    parse_tree("tmp.xml");
}

fn parse_tree(file_path: &str) {
    println!("Enter parsing");
    le_rovine_perdute_rs::parse_map(file_path);    
    println!("finishing parsing");
}
