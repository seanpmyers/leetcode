pub struct Solution;

// Definition for a binary tree node.
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
impl Solution {
    pub fn merge_trees(
        mut root1: Option<Rc<RefCell<TreeNode>>>,
        mut root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(mut x), Some(mut y)) => {
                let mut left =
                    Self::merge_trees(x.borrow_mut().left.take(), y.borrow_mut().left.take());
                let mut right =
                    Self::merge_trees(x.borrow_mut().right.take(), y.borrow_mut().right.take());

                x.borrow_mut().val += y.borrow().val;
                x.borrow_mut().left = left;
                x.borrow_mut().right = right;
                Some(x)
            }
        }
    }
}
