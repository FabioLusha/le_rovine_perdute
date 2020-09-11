pub mod node{
    mod id_giver{
        struct Id_giver{
            id: u32,
        }
        
        impl Id_giver{
            pub fn new() -> Id_giver{
                Id_giver{
                    id: 0,
                }
            }

            pub fn get_id(&self) -> u32 {
                *self += 1;
                *self
            }

        }
    }

    struct Node {
        id: i32,
        adj_nodes: Vec<i32>,
    }

    impl Node {
        fn new(id: u32, adj_nodes: Vec<i32>) -> Node {
            Node{
                id,
                adj_nodes,
            }
        }

        fn add_adj_node(&self, new_adj: &u32) {
            (*adj_nodes).push(*new_adj);
        }

        fn set_id(&self, new_id: u32) {
            *self.id = new_id;
        }

    }
}
