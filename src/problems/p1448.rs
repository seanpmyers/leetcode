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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count: i32 = 0;
        match root {
            Some(r) => {
                pub fn recurse(count: &mut i32, node: Rc<RefCell<TreeNode>>, max: i32) {
                    if node.borrow().val >= max {
                        *count += 1;
                    }
                    let max = max.max(node.borrow().val);
                    match node.borrow_mut().left.take() {
                        Some(l) => recurse(count, l, max),
                        None => {}
                    }
                    match node.borrow_mut().right.take() {
                        Some(r) => recurse(count, r, max),
                        None => {}
                    }
                }
                let max: i32 = r.borrow().val;
                recurse(&mut count, r, max);
            }
            None => {}
        }
        count
    }
}
