mod node;
mod tree;
use tree::Tree;

fn main() {
    let mut tree = Tree::new();
    tree.insert(13);
    tree.insert(12);
    tree.insert(14);
    tree.insert(15);
    tree.insert(16);
    tree.insert(17);
    tree.insert(21);
    tree.insert(1);

    match tree.search(17) {
        Some(node) => println!("Found value: {}\n", node.borrow()),
        None => println!("Value 17 not found"),
    }

    match tree.search(19) {
        Some(node) => println!("Found value: {}\n", node.borrow()),
        None => println!("Value 19 not found"),
    }

    println!("{}", tree);
}
