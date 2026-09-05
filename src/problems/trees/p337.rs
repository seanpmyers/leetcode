pub mod first {

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
        pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let result = dfs(root);
            result.0.max(result.1)
        }
    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        let Some(root) = root else {
            return (0, 0);
        };
        let x = root.borrow().val;
        let left = dfs(root.borrow_mut().left.take());

        let right = dfs(root.borrow_mut().right.take());
        (
            (x + left.1 + right.1).max(left.0 + right.0),
            left.0 + right.0,
        )
    }
}
