pub mod reversal_dfs {
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
        pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let Some(root) = root else {
                return vec![];
            };

            let mut result: Vec<i32> = Vec::new();

            let mut stack: Vec<(Rc<RefCell<TreeNode>>, bool)> = vec![(root, false)];

            while let Some((node, checked)) = stack.pop() {
                if checked {
                    result.push(node.borrow().val);
                    continue;
                }

                if let Some(right) = node.borrow().right.clone() {
                    stack.push((right, false));
                }

                stack.push((node.clone(), true));

                if let Some(left) = node.borrow().left.clone() {
                    stack.push((left, false));
                }
            }

            result.reverse();
            result
        }
    }
}

pub mod iterative {
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
        pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let Some(root) = root else {
                return vec![];
            };

            let mut result: Vec<i32> = Vec::new();

            let mut stack: Vec<(Rc<RefCell<TreeNode>>, bool)> = vec![(root, false)];

            while let Some((node, checked)) = stack.pop() {
                if checked {
                    result.push(node.borrow().val);
                    continue;
                }

                if let Some(left) = node.borrow().left.clone() {
                    stack.push((left, false));
                }

                if let Some(right) = node.borrow().right.clone() {
                    stack.push((right, false));
                }

                stack.push((node.clone(), true));
            }

            result.reverse();
            result
        }
    }
}
