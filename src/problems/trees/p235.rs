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
        pub fn lowest_common_ancestor(
            mut root: Option<Rc<RefCell<TreeNode>>>,
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            let Some(p) = &p else {
                panic!("P and Q must exist");
            };

            let Some(q) = &q else {
                panic!("P and Q must exist");
            };

            let p = p.as_ref().borrow().val;
            let q = q.as_ref().borrow().val;

            while let Some(node) = root.take() {
                let current = node.borrow().val;
                if current > p && current > q {
                    root = node.borrow().left.clone();
                    continue;
                }

                if current < p && current < q {
                    root = node.borrow().right.clone();
                    continue;
                }

                return Some(node);
            }

            None
        }
    }
}

pub mod backtracking {
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
        pub fn lowest_common_ancestor(
            root: Option<Rc<RefCell<TreeNode>>>,
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            let (Some(p), Some(q)) = (p, q) else {
                return root;
            };

            let mut result: (Option<Rc<RefCell<TreeNode>>>, u16) = (root.clone(), 1u16);
            let p_val = p.borrow().val;
            let q_val = q.borrow().val;

            Self::backtrack(root, 1u16, (p_val, q_val), &mut result);

            result.0
        }

        pub fn backtrack(
            root: Option<Rc<RefCell<TreeNode>>>,
            depth: u16,
            (p, q): (i32, i32),
            result: &mut (Option<Rc<RefCell<TreeNode>>>, u16),
        ) -> (bool, bool) {
            let Some(root) = root else {
                return (false, false);
            };
            let mut has_p: bool = root.borrow().val == p;
            let mut has_q: bool = root.borrow().val == q;

            let left_result = Self::backtrack(
                root.borrow().left.clone(),
                depth.saturating_add(1),
                (p, q),
                result,
            );
            has_p = has_p || left_result.0;
            has_q = has_q || left_result.1;
            let right_result = Self::backtrack(
                root.borrow().right.clone(),
                depth.saturating_add(1),
                (p, q),
                result,
            );
            has_p = has_p || right_result.0;
            has_q = has_q || right_result.1;

            if depth > result.1 && has_p && has_q {
                *result = (Some(root.clone()), depth);
            }

            (has_p, has_q)
        }
    }
}
