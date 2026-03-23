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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let Some(root) = root else {
            return false;
        };

        let mut stack: Vec<(Rc<RefCell<TreeNode>>, i32)> = vec![(root, 0)];

        while let Some((node, sum)) = stack.pop() {
            let n = node.borrow();
            let sum: i32 = sum.saturating_add(n.val);
            if sum == target_sum && n.left.is_none() && n.right.is_none() {
                return true;
            }

            if let Some(left) = n.left.clone() {
                stack.push((left, sum))
            }
            if let Some(right) = n.right.clone() {
                stack.push((right, sum))
            }
        }

        false
    }
}
