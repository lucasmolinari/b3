#![allow(dead_code, unused)]

mod tree;
use tree::Tree;

fn main() {
    let mut tree = Tree::new();
    tree.insert(21);
    tree.insert(15);
    tree.insert(16);
    tree.insert(23);
    dbg!(tree);
}
