pub mod dfs {
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
        pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let result: (i32, i32) = dfs(root);
            result.0.max(result.1)
        }
    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        let Some(root) = root else {
            return (0i32, 0i32);
        };

        let mut result: (i32, i32) = (0, 0);
        let left: (i32, i32) = dfs(root.borrow().left.clone());
        let right: (i32, i32) = dfs(root.borrow().right.clone());
        let left_add = left.0
            + if root.borrow().left.is_some() {
                1i32
            } else {
                0
            };
        let right_add = right.0
            + if root.borrow().right.is_some() {
                1i32
            } else {
                0
            };

        result.0 = (left_add).max(right_add);
        result.1 = result.1.max(left.1).max(right.1).max(left_add + right_add);

        result
    }
}
pub mod first {
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
    use std::cell::RefCell;
    use std::rc::Rc;
    impl Solution {
        pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let result = dive(&root);
            result.1
        }
    }

    pub fn dive(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            Some(node) => {
                let n = node.borrow();
                let (l, x) = dive(&n.left);
                let (r, y) = dive(&n.right);
                let depth = x.max(y).max(l + r);
                if l > r {
                    (l + 1, depth)
                } else {
                    (r + 1, depth)
                }
            }
            None => (0, 0),
        }
    }
}
