use std::{cell::RefCell, rc::Rc};

pub type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub lft: Option<NodeRef>,
    pub rgt: Option<NodeRef>,
}
impl Node {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            lft: None,
            rgt: None,
        }
    }

    pub fn height(&self) -> u64 {
        let lft_h = match &self.lft {
            Some(node) => node.borrow().height(),
            None => 0,
        };

        let rgt_h = match &self.rgt {
            Some(node) => node.borrow().height(),
            None => 0,
        };

        std::cmp::max(lft_h, rgt_h) + 1
    }
}
impl From<Node> for Option<NodeRef> {
    fn from(node: Node) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}
