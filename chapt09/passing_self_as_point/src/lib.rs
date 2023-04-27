use std::rc::Rc;

struct Node {
    tag: String,
    children: Vec<Rc<Node>>
}

impl Node {
    fn new(tag: &str) -> Node {
        Node {
            tag: tag.to_string(),
            children: vec![],
        }
    }        

    fn append_to(self: Rc<Self>, parent: &mut Node) {
        parent.children.push(self);
    }
}

#[test]
fn test_append_to() {
    let mut parent = Node::new("parent");
    let shared_node = Rc::new(Node::new("first"));
    shared_node.append_to(&mut parent);
    assert_eq!(parent.children[0].tag, "first");
    //println!("shared_node tag: {}", shared_node.tag); // error, moving

}

#[test]
fn test_append_to_clone() {
    let mut parent = Node::new("parent");
    let shared_node = Rc::new(Node::new("first"));
    // If the caller needs to retain a pointer to the node for later use, then it can clone the Rc
    // first
    shared_node.clone().append_to(&mut parent);
    assert_eq!(parent.children[0].tag, "first");
    println!("shared_node tag: {}", shared_node.tag); 

}

#[test]
fn test_owned_append_to() {
    let mut parent = Node::new("parent");
    let owned = Node::new("owned directly");
    //error: `owned.append_to(&mut parent)`
    Rc::new(owned).append_to(&mut parent);
    assert_eq!(parent.children[0].tag, "owned directly");
}
