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
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let result = dive(&root);
        result.1
    }
}

pub fn dive(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    match root {
        Some(node) => {
            let n = node.borrow();
            let (l, x) = dive(&n.left);
            let (r, y) = dive(&n.right);
            let depth = x.max(y).max(l + r);
            if l > r {
                (l + 1, depth)
            } else {
                (r + 1, depth)
            }
        }
        None => (0, 0),
    }
}
