use graph_test::{Graphadj, Node};

fn main() {
    let mut g = Graphadj::new(2);
    let v1 = Node::new(0, "v1".to_string());
    let v2 = Node::new(0, "v2".to_string());

    g.insert_edge(v1, v2);
    println!("{:?}", g);
}
