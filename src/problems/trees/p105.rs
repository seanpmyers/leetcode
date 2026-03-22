pub mod morris_traversal_iterative {
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
        pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() {
                return None;
            }

            let root: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(0i32)));

            let mut i: usize = 0;
            let mut j: usize = 0;
            let n: usize = preorder.len();
            let mut current: Rc<RefCell<TreeNode>> = root.clone();

            while i < n && j < n {
                let head = Rc::new(RefCell::new(TreeNode::new(preorder[i])));
                head.borrow_mut().right = current.borrow().right.clone();
                current.borrow_mut().right = Some(head.clone());
                current = head;

                i += 1;
                while i < n && current.borrow().val != inorder[j] {
                    let left = Rc::new(RefCell::new(TreeNode {
                        left: None,
                        right: Some(current.clone()),
                        val: preorder[i],
                    }));
                    current.borrow_mut().left = Some(left.clone());
                    current = left;
                    i += 1;
                }

                j += 1;
                while current
                    .borrow()
                    .right
                    .as_ref()
                    .is_some_and(|r| r.borrow().val == inorder[j])
                    && j < n
                {
                    let previous = current.borrow_mut().right.take().unwrap();
                    current = previous;
                    j += 1;
                }
            }

            root.borrow().right.clone()
        }
    }
}

pub mod recursive {
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
        pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() || inorder.is_empty() {
                return None;
            }

            let root = Rc::new(RefCell::new(TreeNode {
                val: preorder[0],
                left: None,
                right: None,
            }));

            let mid: usize = inorder
                .iter()
                .enumerate()
                .find(|(_, x)| preorder[0] == **x)
                .map(|(i, _)| i)
                .unwrap();

            root.borrow_mut().left =
                Self::build_tree(preorder[1..=mid].to_vec(), inorder[0..mid].to_vec());
            root.borrow_mut().right = Self::build_tree(
                preorder[(mid + 1)..preorder.len()].to_vec(),
                inorder[(mid + 1)..inorder.len()].to_vec(),
            );

            Some(root)
        }
    }
}
