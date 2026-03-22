pub mod queue {
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
    use std::collections::VecDeque;
    use std::rc::Rc;
    pub struct Solution;
    impl Solution {
        pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            let Some(root) = root else {
                return vec![];
            };
            let mut result: Vec<Vec<i32>> = vec![];
            let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, usize)> =
                VecDeque::from(vec![(root, 0)]);

            while let Some((node, depth)) = queue.pop_front() {
                let n = node.borrow();
                if result.len() < depth + 1 {
                    result.push(vec![]);
                }
                result[depth].push(n.val);

                if let Some(left) = n.left.clone() {
                    queue.push_back((left, depth + 1));
                }

                if let Some(right) = n.right.clone() {
                    queue.push_back((right, depth + 1));
                }
            }

            result
        }
    }
}

pub mod stack {
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
    pub struct Solution {}
    impl Solution {
        pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = vec![];
            if root.is_none() {
                return result;
            }
            let mut stack: Vec<(Rc<RefCell<TreeNode>>, usize)> = vec![(root.unwrap(), 0)];
            while let Some((node, level)) = stack.pop() {
                if result.len() <= level {
                    result.push(vec![]);
                }
                result[level].push(node.as_ref().borrow().val);
                if let Some(right) = node.as_ref().borrow().right.clone() {
                    stack.push((right, level + 1));
                }
                if let Some(left) = node.as_ref().borrow().left.clone() {
                    stack.push((left, level + 1));
                }
            }

            result
        }
    }
}
