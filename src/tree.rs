use std::fmt::{Display, Formatter};

use crate::node::{Node, NodeRef};

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
            Some(root) => Tree::irec(root, value),
            None => self.root = Node::new(value).into(),
        };
    }

    fn irec(root: &mut NodeRef, value: i32) {
        let mut parent = root.borrow_mut();

        if value < parent.value {
            match &mut parent.lft {
                Some(lft) => Tree::irec(lft, value),
                None => parent.lft = Node::new(value).into(),
            }
        }
        if value > parent.value {
            match &mut parent.rgt {
                Some(rgt) => Tree::irec(rgt, value),
                None => parent.rgt = Node::new(value).into(),
            }
        }

        drop(parent);
        Tree::rotate(root, value);
    }

    fn rotate(root: &mut NodeRef, value: i32) {
        let node = root.borrow();

        let (lft_h, lft_v) = match &node.lft {
            Some(l) => (l.borrow().height() as i64, l.borrow().value),
            None => (0, 0),
        };

        let (rgt_h, rgt_v) = match &node.rgt {
            Some(r) => (r.borrow().height() as i64, r.borrow().value),
            None => (0, 0),
        };

        drop(node);
        let balance = lft_h - rgt_h;
        if balance > 1 {
            if value < lft_v {
                Tree::r_rotation(root);
            } else {
                Tree::lr_rotation(root);
            }
        }

        if balance < -1 {
            if value > rgt_v {
                Tree::l_rotation(root);
            } else {
                Tree::rl_rotation(root);
            }
        }
    }

    fn l_rotation(root: &mut NodeRef) {
        let mut z = root.borrow_mut();
        let yrc = z.rgt.as_ref().expect("Should have Right Child").clone();
        let mut y = yrc.borrow_mut();

        z.rgt = y.lft.take();
        drop(z);

        y.lft = Some(root.clone());
        drop(y);
        *root = yrc.clone();
    }

    fn r_rotation(root: &mut NodeRef) {
        let mut z = root.borrow_mut();
        let yrc = z.lft.as_ref().expect("Should have Left Child").clone();
        let mut y = yrc.borrow_mut();

        z.lft = y.rgt.take();
        drop(z);

        y.rgt = Some(root.clone());
        drop(y);
        *root = yrc.clone();
    }

    fn lr_rotation(root: &mut NodeRef) {
        let mut z = root.borrow_mut();
        let mut xrc = z.lft.as_ref().expect("Should have Left Child").clone();

        Tree::l_rotation(&mut xrc);
        z.lft = Some(xrc);
        drop(z);
        Tree::r_rotation(root);
    }

    fn rl_rotation(root: &mut NodeRef) {
        let mut z = root.borrow_mut();
        let mut xrc = z.rgt.as_ref().expect("Should have Right Child").clone();

        Tree::r_rotation(&mut xrc);
        z.rgt = Some(xrc);

        drop(z);
        Tree::l_rotation(root)
    }
}

type FmtResult = std::fmt::Result;
impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fn print_empty(f: &mut Formatter<'_>, prefix: &str, is_left: bool) -> FmtResult {
            writeln!(f, "{}{}()", prefix, if is_left { "├── " } else { "└── " })?;
            Ok(())
        }
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

            if node.lft.is_none() && node.rgt.is_none() {
                return Ok(());
            }

            if let Some(left) = &node.lft {
                print_tree(left, f, &new_prefix, true)?;
            } else {
                print_empty(f, &new_prefix, true)?;
            }
            if let Some(right) = &node.rgt {
                print_tree(right, f, &new_prefix, false)?;
            } else {
                print_empty(f, &new_prefix, false)?;
            }

            Ok(())
        }

        if let Some(root) = &self.root {
            print_tree(root, f, "", false)?;
        }

        Ok(())
    }
}
