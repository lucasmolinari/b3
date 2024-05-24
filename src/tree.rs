type NodeRef = Box<Node>;

#[derive(Debug)]
pub struct Tree {
    root: Option<NodeRef>,
}
impl Tree {
    pub fn new() -> Self{
        Self { root: None }
    }
    pub fn insert(&mut self, value: i32) {
        match &mut self.root {
            Some(node) => Tree::rec(node, value),
            None => self.root = Node::new(value).into(),
        };
    }

    fn rec(node: &mut NodeRef, value: i32) {
        if value < node.value {
            match &mut node.lft {
                Some(n) => Tree::rec(n, value),
                None => node.lft = Node::new(value).into(),
            }
        }
        if value > node.value {
            match &mut node.rgt {
                Some(n) => Tree::rec(n, value),
                None => node.rgt = Node::new(value).into(),
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    lft: Option<NodeRef>,
    rgt: Option<NodeRef>,
}
impl Node {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            lft: None,
            rgt: None,
        }
    }
}
impl From<Node> for Option<NodeRef> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

