pub mod dfs {
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
        pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut result: i32 = -1000;
            let max = dfs(root, &mut result);

            result.max(max)
        }
    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        let Some(root) = root else {
            return 0;
        };
        let value = root.borrow().val;
        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();

        let left = dfs(left, max);
        let right = dfs(right, max);
        let result: i32 = left.max(right) + value;

        *max = result.max(*max).max(left + right + value).max(value);
        result.max(value)
    }
}
pub mod first {
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
        pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut global: i32 = i32::MIN;

            Self::max(root, &mut global).max(global)
        }

        pub fn max(root: Option<Rc<RefCell<TreeNode>>>, global: &mut i32) -> i32 {
            let Some(root) = root else {
                return i32::MIN;
            };

            let rb = root.as_ref().borrow();
            let max: i32 = rb.val;
            let left: i32 = Self::max(rb.left.clone(), global);
            let right: i32 = Self::max(rb.right.clone(), global);

            *global = right
                .max(left)
                .max(max.saturating_add(right).saturating_add(left))
                .max(*global);

            max.max(left.saturating_add(max))
                .max(right.saturating_add(max))
        }
    }
}
