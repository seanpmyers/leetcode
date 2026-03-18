pub mod first_attempt {
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
    pub struct Solution;
    use std::cell::RefCell;
    use std::cmp::Ordering;
    use std::rc::Rc;
    impl Solution {
        pub fn delete_node(
            mut root: Option<Rc<RefCell<TreeNode>>>,
            key: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if root.is_none() {
                return None;
            }

            let mut current = root.clone();
            let mut previous = None;
            while let Some(node) = current {
                let mut n = node.borrow_mut();
                match key.cmp(&n.val) {
                    Ordering::Greater => {
                        current = n.right.clone();
                        previous = Some(node.clone());
                        continue;
                    }
                    Ordering::Less => {
                        current = n.left.clone();
                        previous = Some(node.clone());
                        continue;
                    }
                    Ordering::Equal => {
                        let mut left = n.left.take();
                        let right = n.right.take();
                        let r_clone = right.clone();
                        let mut l_clone = left.clone();
                        while let Some(l_iter) = l_clone {
                            let mut l_c_b = l_iter.borrow_mut();
                            if l_c_b.right.is_none() {
                                l_c_b.right = right;
                                break;
                            }

                            l_clone = l_c_b.right.clone();
                        }

                        if left.is_none() {
                            left = r_clone;
                        }

                        let Some(p) = previous else {
                            root = left;
                            break;
                        };

                        let mut prev = p.borrow_mut();

                        match prev.val < key {
                            true => prev.right = left,
                            false => prev.left = left,
                        };

                        break;
                    }
                }
            }

            root
        }
    }
}
