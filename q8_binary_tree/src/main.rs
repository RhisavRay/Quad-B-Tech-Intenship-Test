use std::rc::Rc;
use std::cell::RefCell;

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
    // Example binary tree
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));

    // Calculate the maximum depth of the tree
    let depth = max_depth(root);
    println!("Maximum depth of the tree: {}", depth); // Output: Maximum depth of the tree: 3
}
