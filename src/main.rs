use node::{self, id_giver};

fn main() {
    let id_creater = node::Id_giver::new();
    let nod = node::Node::new(id_creater.get_id, 0);
}
