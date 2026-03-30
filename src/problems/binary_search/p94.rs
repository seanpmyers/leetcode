pub mod optimal {
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
        pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let mut result: Vec<i32> = vec![];
            let mut current = root;
            let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
            while current.is_some() || !stack.is_empty() {
                while let Some(node) = current {
                    current = node.borrow().left.clone();
                    stack.push(node);
                }

                if let Some(node) = stack.pop() {
                    result.push(node.borrow().val);
                    current = node.borrow().right.clone();
                }
            }

            result
        }
    }
}

pub mod mutate_tree {
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
        pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            let Some(root) = root else {
                return vec![];
            };
            let mut result: Vec<i32> = Vec::new();
            let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![root];
            while let Some(node) = stack.pop() {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                if let Some(right) = right {
                    stack.push(right);
                }

                if let Some(left) = left {
                    stack.push(node);
                    stack.push(left);
                    continue;
                }

                result.push(node.borrow().val);
            }

            result
        }
    }
}
