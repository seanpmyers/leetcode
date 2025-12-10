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
        let Some(root) = root else {
            return true;
        };

        use std::collections::VecDeque;

        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, Option<i32>, Option<i32>)> =
            VecDeque::with_capacity(10usize.pow(4u32));

        queue.push_back((root.clone(), None, None));

        while let Some((node, min, max)) = queue.pop_front() {
            let n_value = node.borrow().val;

            if min.is_some_and(|m| n_value <= m) {
                return false;
            }

            if max.is_some_and(|m| n_value >= m) {
                return false;
            }

            if let Some(left) = node.borrow().left.clone() {
                queue.push_back((left, min, Some(n_value)));
            }

            if let Some(right) = node.borrow().right.clone() {
                queue.push_back((right, Some(n_value), max));
            }
        }

        true
    }
}
