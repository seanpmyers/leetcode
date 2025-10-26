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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode {
            val: preorder[0],
            left: None,
            right: None,
        }));

        let mid: usize = inorder
            .iter()
            .enumerate()
            .find(|(_, x)| preorder[0] == **x)
            .map(|(i, _)| i)
            .unwrap();

        root.borrow_mut().left =
            Self::build_tree(preorder[1..=mid].to_vec(), inorder[0..mid].to_vec());
        root.borrow_mut().right = Self::build_tree(
            preorder[(mid + 1)..preorder.len()].to_vec(),
            inorder[(mid + 1)..inorder.len()].to_vec(),
        );

        Some(root)
    }
}
