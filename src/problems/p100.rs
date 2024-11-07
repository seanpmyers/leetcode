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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut stack: Vec<(Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>)> =
            vec![(p, q)];
        while let Some((x, y)) = stack.pop() {
            match (x, y) {
                (Some(p), Some(q)) => {
                    if p.as_ref().borrow().val != q.as_ref().borrow().val {
                        return false;
                    }
                    stack.push((
                        p.as_ref().borrow_mut().left.take(),
                        q.as_ref().borrow_mut().left.take(),
                    ));
                    stack.push((
                        p.as_ref().borrow_mut().right.take(),
                        q.as_ref().borrow_mut().right.take(),
                    ));
                }
                (None, None) => continue,
                (None, Some(_)) => return false,
                (Some(_), None) => return false,
            }
        }
        true
    }
}
