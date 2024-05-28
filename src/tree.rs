use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    rc::Rc,
};

type NodeRef = Rc<RefCell<Node>>;

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
        Some(Rc::new(RefCell::new(node)))
    }
}

#[derive(Debug)]
pub struct Tree {
    root: Option<NodeRef>,
}
impl Tree {
    pub fn new() -> Self {
        Self { root: None }
    }
    pub fn insert(&mut self, value: i32) {
        match &mut self.root {
            Some(node) => Tree::irec(node, value),
            None => self.root = Node::new(value).into(),
        };
    }

    fn irec(node: &mut NodeRef, value: i32) {
        let mut node = node.borrow_mut();
        if value < node.value {
            match &mut node.lft {
                Some(n) => Tree::irec(n, value),
                None => {
                    node.lft = Node::new(value).into();
                }
            }
        }
        if value > node.value {
            match &mut node.rgt {
                Some(n) => Tree::irec(n, value),
                None => {
                    node.rgt = Node::new(value).into();
                }
            }
        }
    }
}

type FmtResult = std::fmt::Result;
impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fn print_tree(
            node: &NodeRef,
            f: &mut Formatter<'_>,
            prefix: &str,
            is_left: bool,
        ) -> FmtResult {
            let node = node.borrow_mut();
            writeln!(
                f,
                "{}{}{}",
                prefix,
                if is_left { "├── " } else { "└── " },
                node.value
            )?;

            let new_prefix = if is_left {
                format!("{}│   ", prefix)
            } else {
                format!("{}    ", prefix)
            };

            if let Some(left) = &node.lft {
                print_tree(left, f, &new_prefix, true)?;
            }
            if let Some(right) = &node.rgt {
                print_tree(right, f, &new_prefix, false)?;
            }

            Ok(())
        }

        if let Some(root) = &self.root {
            print_tree(root, f, "", false)?;
        }

        Ok(())
    }
}
