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
pub struct Solution;
impl Solution {
    pub fn lowest_common_ancestor(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(p) = &p else {
            panic!("P and Q must exist");
        };

        let Some(q) = &q else {
            panic!("P and Q must exist");
        };

        let p = p.as_ref().borrow().val;
        let q = q.as_ref().borrow().val;

        while let Some(node) = root.take() {
            let current = node.borrow().val;
            if current > p && current > q {
                root = node.borrow().left.clone();
                continue;
            }

            if current < p && current < q {
                root = node.borrow().right.clone();
                continue;
            }

            return Some(node);
        }

        None
    }
}
