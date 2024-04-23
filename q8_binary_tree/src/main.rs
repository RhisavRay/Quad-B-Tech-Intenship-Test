use std::rc::Rc;
use std::cell::RefCell;
use std::io;

// Definition of a binary tree node
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Function to construct a binary tree from user input
fn construct_tree() -> Option<Rc<RefCell<TreeNode>>> {
    println!("Enter the value of the node (or -1 to represent null):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let val: i32 = input.trim().parse().expect("Please enter a valid integer");

    if val == -1 {
        return None; // Null node
    }

    let node = Rc::new(RefCell::new(TreeNode::new(val)));

    println!("Enter the left child of node {}: ", val);
    let left_child = construct_tree();
    node.borrow_mut().left = left_child;

    println!("Enter the right child of node {}: ", val);
    let right_child = construct_tree();
    node.borrow_mut().right = right_child;

    Some(node)
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let node = node.borrow();
            let left_depth = max_depth(node.left.clone());
            let right_depth = max_depth(node.right.clone());
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {

    println!("Construct the binary tree:");
    let root = construct_tree();

    // Calculate the maximum depth of the tree
    let depth = max_depth(root);
    println!("Maximum depth of the tree: {}", depth);
}