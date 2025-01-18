use std::cell::RefCell;
use std::rc::Rc;
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
pub struct Solution;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(r) => {
                pub fn dive(n: Rc<RefCell<TreeNode>>, max: Option<i32>, min: Option<i32>) -> bool {
                    let n_val = n.as_ref().borrow().val;
                    if let Some(l) = min {
                        if n_val <= l {
                            return false;
                        }
                    }
                    if let Some(r) = max {
                        if n_val >= r {
                            return false;
                        }
                    }

                    if let Some(l) = n.as_ref().borrow().left.clone() {
                        if !dive(l, Some(n_val), min) {
                            return false;
                        }
                    }
                    if let Some(r) = n.as_ref().borrow().right.clone() {
                        if !dive(r, max, Some(n_val)) {
                            return false;
                        }
                    }
                    true
                }
                dive(r, None, None)
            }
            None => true,
        }
    }
}
