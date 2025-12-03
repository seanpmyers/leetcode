use std::{cell::RefCell, rc::Rc};

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(root).1
    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (u16, bool) {
        let Some(root) = root else {
            return (0u16, true);
        };

        let (left, l_valid) = Self::dfs(root.borrow().left.clone());
        let (right, r_valid) = Self::dfs(root.borrow().right.clone());

        (
            1u16 + left.max(right),
            l_valid && r_valid && left.abs_diff(right) <= 1u16,
        )
    }
}
