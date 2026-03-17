pub struct Solution;
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
impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode {
                left: None,
                right: None,
                val: val,
            })));
        }
        let mut current = root.clone();

        while let Some(node) = current {
            let mut n = node.borrow_mut();

            match n.val < val {
                true => {
                    if n.right.is_some() {
                        current = n.right.clone();
                        continue;
                    }
                    n.right = Some(Rc::new(RefCell::new(TreeNode {
                        left: None,
                        right: None,
                        val: val,
                    })));
                }
                false => {
                    if n.left.is_some() {
                        current = n.left.clone();
                        continue;
                    }
                    n.left = Some(Rc::new(RefCell::new(TreeNode {
                        left: None,
                        right: None,
                        val: val,
                    })));
                }
            }
            break;
        }

        root
    }
}
