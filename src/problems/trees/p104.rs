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

pub struct Solution {}
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        memory_optimized(root)
    }
}

pub fn memory_optimized(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let mut max: i32 = 0;

    let mut stack: Vec<(i32, Rc<RefCell<TreeNode>>)> = vec![(1, root.unwrap())];
    while let Some((h, n)) = stack.pop() {
        max = max.max(h);
        if let Some(left) = n.as_ref().borrow().left.clone() {
            stack.push((h + 1, left))
        }
        if let Some(right) = n.as_ref().borrow().right.clone() {
            stack.push((h + 1, right))
        }
    }
    max
}

pub fn recursive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left: i32 = recursive(node.as_ref().borrow().left.clone());
            let right: i32 = recursive(node.as_ref().borrow().right.clone());
            1 + left + right
        }
    }
}
