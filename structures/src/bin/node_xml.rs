use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Node {
    tag: String,
    children: Vec<Rc<Node>>,
}

impl Node {
    pub fn new(tag: &str) -> Node {
        Node {
            tag: tag.to_string(),
            children: vec![],
        }
    }

    fn append_to(self: Rc<Self>, parent: &mut Node) {
        parent.children.push(self);
    }
}

fn main() {
    let mut parent = Node::new("parent");
    let shared_node = Rc::new(Node::new("first"));
    shared_node.append_to(&mut parent);
    dbg!(parent);
    // shared_node desinicializado

    let mut parent = Node::new("parent");
    let shared_node = Rc::new(Node::new("first"));
    shared_node.clone().append_to(&mut parent);
    dbg!(parent);
    dbg!(shared_node);
    // shared_node se mantiene ya que se clono antes de moverlo a parent

    let mut parent = Node::new("parent");
    Rc::new(Node::new("first")).append_to(&mut parent);

    println!("Proceso finalizado");
}
