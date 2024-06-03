use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    rc::Rc,
};

#[derive(Debug)]
enum Rotation {
    L,
    R,
    LR,
    RL,
}

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

    fn height(&self) -> u64 {
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
            Some(root) => {
                match Tree::irec(root, value) {
                    Some(Rotation::L) => Tree::l_rotation(root),
                    Some(Rotation::R) => Tree::r_rotation(root),
                    Some(Rotation::LR) => println!("Rotation Left Right"),
                    Some(Rotation::RL) => println!("Rotation Right Left"),
                    None => println!("No Rotation"),
                };
            }
            None => self.root = Node::new(value).into(),
        };
    }

    fn irec(parent: &mut NodeRef, value: i32) -> Option<Rotation> {
        let mut parent = parent.borrow_mut();

        if value < parent.value {
            match &mut parent.lft {
                Some(lft) => _ = Tree::irec(lft, value),
                None => {
                    parent.lft = Node::new(value).into();
                    return None;
                }
            }
        }
        if value > parent.value {
            match &mut parent.rgt {
                Some(rgt) => _ = Tree::irec(rgt, value),
                None => {
                    parent.rgt = Node::new(value).into();
                    return None;
                }
            }
        }

        let (lft_h, lft_v) = match &parent.lft {
            Some(l) => (l.borrow().height() as i64, l.borrow().value),
            None => (0, 0),
        };

        let (rgt_h, rgt_v) = match &parent.rgt {
            Some(r) => (r.borrow().height() as i64, r.borrow().value),
            None => (0, 0),
        };

        println!(
            "Left Value: {} | Right Value: {} | Value: {}",
            lft_v, rgt_v, value
        );
        if lft_h - rgt_h > 1 {
            if lft_v > value {
                println!("Returning Rotation R");
                return Some(Rotation::R);
            } else {
                println!("Returning Rotation LR");
                return Some(Rotation::LR);
            }
        }

        if lft_h - rgt_h < -1 {
            if rgt_v < value {
                println!("Returning Rotation L");
                return Some(Rotation::L);
            } else {
                println!("Returning Rotation RL");
                return Some(Rotation::RL);
            }
        }

        None
    }

    fn l_rotation(root: &mut NodeRef) {
        let yrc;

        {
            let mut z = root.borrow_mut();
            yrc = z.rgt.as_ref().expect("Should have Right Child").clone();
            let mut y = yrc.borrow_mut();
            z.rgt = y.lft.take();
        }

        let mut y = yrc.borrow_mut();
        y.lft = Some(root.clone());
        *root = yrc.clone();
    }

    fn r_rotation(root: &mut NodeRef) {
        let yrc;

        {
            let mut z = root.borrow_mut();
            yrc = z.lft.as_ref().expect("Should have Left Child").clone();
            let mut y = yrc.borrow_mut();
            z.lft = y.rgt.take();
        }

        let mut y = yrc.borrow_mut();
        y.rgt = Some(root.clone());
        *root = yrc.clone();
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
