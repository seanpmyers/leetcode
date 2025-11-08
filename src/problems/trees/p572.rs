pub mod dfs {
    pub struct Solution;
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
        pub fn is_subtree(
            root: Option<Rc<RefCell<TreeNode>>>,
            sub_root: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            let Some(root) = root else {
                return false;
            };

            let Some(sub_root) = sub_root else {
                return false;
            };

            if Self::same_tree(Some(root.clone()), Some(sub_root.clone())) {
                return true;
            }

            Self::is_subtree(root.borrow().left.clone(), Some(sub_root.clone()))
                || Self::is_subtree(root.borrow().right.clone(), Some(sub_root.clone()))
        }

        pub fn same_tree(
            root: Option<Rc<RefCell<TreeNode>>>,
            sub_root: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (root, sub_root) {
                (None, None) => true,
                (None, Some(_)) | (Some(_), None) => false,
                (Some(root), Some(sub_root)) => {
                    if root.borrow().val != sub_root.borrow().val {
                        return false;
                    };
                    Self::same_tree(root.borrow().left.clone(), sub_root.borrow().left.clone())
                        && Self::same_tree(
                            root.borrow().right.clone(),
                            sub_root.borrow().right.clone(),
                        )
                }
            }
        }
    }
}
