# B3

A self-balancing Binary Tree ([AVL](https://en.wikipedia.org/wiki/AVL_tree)) implemented in Rust 🦀

### Supported operations (WIP)
 - Insertion

**Example:** 
```rust
fn main() {
    let mut tree = Tree::new();
    tree.insert(50);
    tree.insert(61);
    tree.insert(52);
    tree.insert(35);
    tree.insert(18);

    println!("{}", tree);
}
```
**Output:**
```
└── 52
    ├── 35
    │   ├── 18
    │   └── 50
    └── 61
```
