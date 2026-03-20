pub mod optimal {
    use std::{cell::RefCell, rc::Rc};

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

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        use std::collections::VecDeque;

        let mut list: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::with_capacity(k as usize);
        let mut current = root.clone();

        while !list.is_empty() || current.is_some() {
            while let Some(node) = current {
                list.push_back(node.clone());
                current = node.borrow().left.clone();
            }

            current = list.pop_back();
            k = k.saturating_sub(1);

            if k == 0 {
                return current.unwrap().borrow().val;
            }

            current = current.unwrap().borrow().right.clone();
        }

        0i32
    }
}
pub mod binary_heap {
    pub struct Solution;
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
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::rc::Rc;
    impl Solution {
        pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
            let Some(root) = root else { return 0i32 };
            let k: usize = k as usize;
            let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k);
            let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::with_capacity(k * 2);
            stack.push(root);
            while let Some(node) = stack.pop() {
                let n = node.borrow();
                heap.push(Reverse(n.val));
                if let Some(left) = n.left.clone() {
                    stack.push(left);
                }

                if let Some(right) = n.right.clone() {
                    stack.push(right);
                }
            }

            let mut result: i32 = 0;
            for _ in 0..k {
                result = heap.pop().unwrap().0;
            }

            result
        }
    }
}
