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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut global: i32 = i32::MIN;

        Self::max(root, &mut global).max(global)
    }

    pub fn max(root: Option<Rc<RefCell<TreeNode>>>, global: &mut i32) -> i32 {
        let Some(root) = root else {
            return i32::MIN;
        };

        let rb = root.as_ref().borrow();
        let max: i32 = rb.val;
        let left: i32 = Self::max(rb.left.clone(), global);
        let right: i32 = Self::max(rb.right.clone(), global);

        *global = right
            .max(left)
            .max(max.saturating_add(right).saturating_add(left))
            .max(*global);

        max.max(left.saturating_add(max))
            .max(right.saturating_add(max))
    }
}
