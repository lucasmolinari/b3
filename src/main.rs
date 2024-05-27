mod tree;
use tree::Tree;

fn main() {
    let mut tree = Tree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(2);
    tree.insert(4);
    tree.insert(6);
    tree.insert(1);
    tree.insert(8);
    println!("{}", tree);
}
