pub mod iterative {
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
    use std::collections::HashMap;
    use std::rc::Rc;
    pub struct Solution;
    impl Solution {
        pub fn is_balanced(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
            let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
            let mut map: HashMap<usize, i32> = HashMap::new();
            let mut last: Option<usize> = None;
            while !stack.is_empty() || root.is_some() {
                if let Some(node) = root {
                    stack.push(node.clone());
                    root = node.borrow().left.clone();
                    continue;
                }

                let current = stack.last().cloned().unwrap();

                if current.borrow().right.is_some()
                    && current
                        .borrow()
                        .right
                        .clone()
                        .map(|r| Rc::as_ptr(&r) as usize)
                        != last
                {
                    root = current.borrow().right.clone();
                    continue;
                }

                let current = stack.pop().unwrap();
                let mut left_depth: i32 = 0i32;
                let mut right_depth: i32 = 0i32;
                if let Some(left) = current.borrow().left.clone() {
                    if let Some(depth) = map.get(&(Rc::as_ptr(&left) as usize)) {
                        left_depth = *depth;
                    }
                }
                if let Some(right) = current.borrow().right.clone() {
                    if let Some(depth) = map.get(&(Rc::as_ptr(&right) as usize)) {
                        right_depth = *depth;
                    }
                }

                if left_depth.abs_diff(right_depth) > 1 {
                    return false;
                }

                map.insert(
                    Rc::as_ptr(&current) as usize,
                    1 + left_depth.max(right_depth),
                );
                last = Some(Rc::as_ptr(&current) as usize);
                root = None;
            }

            true
        }
    }
}

pub mod recursive {
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

    pub struct Solution;
    impl Solution {
        pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
            Self::dfs(root).1
        }

        pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (u16, bool) {
            let Some(root) = root else {
                return (0u16, true);
            };

            let (left, l_valid) = Self::dfs(root.borrow().left.clone());
            let (right, r_valid) = Self::dfs(root.borrow().right.clone());

            (
                1u16 + left.max(right),
                l_valid && r_valid && left.abs_diff(right) <= 1u16,
            )
        }
    }
}
