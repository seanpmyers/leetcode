#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }
        let root = root.unwrap();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![root.clone()];
        while let Some(node) = stack.pop() {
            let mut node_ref = node.borrow_mut();
            let temp = node_ref.left.take();
            node_ref.left = node_ref.right.take();
            node_ref.right = temp;
            if let Some(left) = node_ref.left.clone() {
                stack.push(left);
            }
            if let Some(right) = node_ref.right.clone() {
                stack.push(right);
            }
        }
        Some(root)
    }
}
