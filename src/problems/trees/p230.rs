use std::{cell::RefCell, rc::Rc};

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

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
    use std::collections::VecDeque;

    let mut list: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::with_capacity(k as usize);
    let mut current = root.clone();

    while !list.is_empty() || current.is_some() {
        while let Some(node) = current {
            list.push_back(node.clone());
            current = node.borrow().left.clone();
        }

        current = list.pop_back();
        k = k.saturating_sub(1);

        if k == 0 {
            return current.unwrap().borrow().val;
        }

        current = current.unwrap().borrow().right.clone();
    }

    0i32
}
