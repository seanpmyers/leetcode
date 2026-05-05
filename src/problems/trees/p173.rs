#[allow(dead_code)]
pub mod vector {
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
    struct BSTIterator {
        pub list: Vec<i32>,
        pub current: usize,
    }

    use std::cell::RefCell;
    use std::rc::Rc;
    impl BSTIterator {
        fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
            let mut list: Vec<i32> = vec![i32::MIN];
            let mut stack: Vec<(Rc<RefCell<TreeNode>>, bool)> =
                vec![(root.clone().unwrap(), false)];

            while let Some((node, checked)) = stack.pop() {
                if checked {
                    list.push(node.borrow().val);
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

            Self { list, current: 0 }
        }

        fn next(&mut self) -> i32 {
            self.current += 1;
            self.list[self.current]
        }

        fn has_next(&self) -> bool {
            if self.current + 2 > self.list.len() {
                return false;
            }

            true
        }
    }
}
