pub mod bfs {

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
    use std::collections::{HashMap, VecDeque};
    use std::rc::Rc;
    pub struct Solution;
    impl Solution {
        pub fn lowest_common_ancestor(
            root: Option<Rc<RefCell<TreeNode>>>,
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            let Some(p) = p else {
                return None;
            };
            let Some(q) = q else {
                return None;
            };
            let x: i32 = p.borrow().val;
            let y: i32 = q.borrow().val;
            let child_of_p = p
                .borrow()
                .left
                .as_ref()
                .is_some_and(|n| n.borrow().val == y)
                || p.borrow()
                    .right
                    .as_ref()
                    .is_some_and(|n| n.borrow().val == y);
            let child_of_q = q
                .borrow()
                .left
                .as_ref()
                .is_some_and(|n| n.borrow().val == x)
                || q.borrow()
                    .right
                    .as_ref()
                    .is_some_and(|n| n.borrow().val == x);
            if child_of_p {
                return Some(p.clone());
            }
            if child_of_q {
                return Some(q.clone());
            }
            let Some(root) = root else {
                return None;
            };
            if root.borrow().val == x || root.borrow().val == y {
                return Some(root);
            }

            let mut map: HashMap<i32, (Rc<RefCell<TreeNode>>, usize, bool, bool)> = HashMap::new();
            let mut list = VecDeque::new();
            list.push_back((root, 0usize, vec![]));

            while let Some((node, depth, mut ancestors)) = list.pop_front() {
                let value: i32 = node.borrow().val;
                map.insert(value, (node.clone(), depth, value == x, value == y));
                if value == x || value == y {
                    for v in ancestors.iter() {
                        map.entry(*v).and_modify(|e| {
                            if value == x {
                                e.2 = true;
                            }
                            if value == y {
                                e.3 = true;
                            }
                        });
                    }
                }

                ancestors.push(value);
                if let Some(left) = node.borrow_mut().left.take() {
                    list.push_back((left, depth + 1, ancestors.clone()));
                }
                if let Some(right) = node.borrow_mut().right.take() {
                    list.push_back((right, depth + 1, ancestors));
                }
            }
            let mut result = (0, None);
            for (_, (node, depth, has_x, has_y)) in map.into_iter() {
                if !has_x || !has_y {
                    continue;
                }
                if depth < result.0 {
                    continue;
                }
                result = (depth, Some(node.clone()));
            }
            result.1
        }
    }
}
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
