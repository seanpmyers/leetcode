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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(r) => {
                let mut stack: Vec<(Rc<RefCell<TreeNode>>, usize)> = vec![(r, 1)];
                let mut result: Vec<i32> = vec![];
                while let Some((top, depth)) = stack.pop() {
                    if result.len() < depth {
                        result.push(top.as_ref().borrow().val);
                    }
                    if top.as_ref().borrow().left.is_some() {
                        stack.push((top.as_ref().borrow_mut().left.take().unwrap(), depth + 1));
                    }
                    if top.as_ref().borrow().right.is_some() {
                        stack.push((top.as_ref().borrow_mut().right.take().unwrap(), depth + 1));
                    }
                }
                result
            }
            None => vec![],
        }
    }
}
