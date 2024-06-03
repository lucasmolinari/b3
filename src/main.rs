mod tree;
use tree::Tree;

fn main() {
    let mut tree = Tree::new();
    tree.insert(10);
    tree.insert(20);
    tree.insert(30);
    tree.insert(40);
    tree.insert(50);
    tree.insert(61);
    tree.insert(52);
    tree.insert(54);
    tree.insert(63);
    tree.insert(15);
    tree.insert(25);
    tree.insert(35);
    tree.insert(18);
    tree.insert(16);
    tree.insert(17);

    println!("{}", tree);
}
