pub mod recursive {
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
        pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut count: i32 = 0;
            match root {
                Some(r) => {
                    pub fn recurse(count: &mut i32, node: Rc<RefCell<TreeNode>>, max: i32) {
                        if node.borrow().val >= max {
                            *count += 1;
                        }
                        let max = max.max(node.borrow().val);
                        match node.borrow_mut().left.take() {
                            Some(l) => recurse(count, l, max),
                            None => {}
                        }
                        match node.borrow_mut().right.take() {
                            Some(r) => recurse(count, r, max),
                            None => {}
                        }
                    }
                    let max: i32 = r.borrow().val;
                    recurse(&mut count, r, max);
                }
                None => {}
            }
            count
        }
    }
}

pub mod stack {
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
        pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut result: i32 = 0i32;
            let Some(root) = root else {
                return result;
            };

            let mut stack: Vec<(i32, Rc<RefCell<TreeNode>>)> =
                Vec::with_capacity(10usize.pow(5u32));

            let min: i32 = root.borrow().val;
            stack.push((min, root));

            while let Some((max, node)) = stack.pop() {
                if node.borrow().val >= max {
                    result = result.saturating_add(1);
                }

                if let Some(left) = node.borrow().left.clone() {
                    stack.push((max.max(node.borrow().val), left));
                };
                if let Some(right) = node.borrow().right.clone() {
                    stack.push((max.max(node.borrow().val), right));
                };
            }

            result
        }
    }
}
