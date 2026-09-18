#[allow(dead_code)]
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
    struct Codec {}

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl Codec {
        fn new() -> Self {
            Self {}
        }

        fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
            let mut result: String = String::new();
            if root.is_none() {
                return result;
            }
            let mut list = VecDeque::new();
            list.push_back(root);
            while let Some(possible_node) = list.pop_front() {
                if !result.is_empty() {
                    result.push(',');
                }
                let Some(node) = possible_node else {
                    result.push('n');
                    continue;
                };
                result.push_str(&node.borrow().val.to_string());
                list.push_back(node.borrow_mut().left.take());
                list.push_back(node.borrow_mut().right.take());
            }

            result
        }

        fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
            if data.is_empty() {
                return None;
            }
            let data: &[u8] = data.as_bytes();
            let mut i: usize = 0;
            while data[i] != b',' {
                i += 1;
            }
            let value = String::from_utf8(data[0..i].to_vec())
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let root = Rc::new(RefCell::new(TreeNode::new(value)));

            let mut queue = VecDeque::new();
            queue.push_back(root.clone());
            while let Some(node) = queue.pop_front() {
                if i >= data.len() {
                    break;
                }
                if data[i] == b',' {
                    i += 1;
                }

                let mut end: usize = i + 1;
                while data[end] != b',' {
                    end += 1;
                }
                let left_text = String::from_utf8(data[i..end].to_vec()).unwrap();
                i = end + 1;
                end = i + 1;
                while end < data.len() - 1 && data[end] != b',' {
                    end += 1;
                }
                let right_text = String::from_utf8(data[i..end].to_vec()).unwrap();
                i = end + 1;

                let left = match left_text.parse::<i32>() {
                    Ok(value) => Some(Rc::new(RefCell::new(TreeNode::new(value)))),
                    Err(_) => None,
                };
                let right = match right_text.parse::<i32>() {
                    Ok(value) => Some(Rc::new(RefCell::new(TreeNode::new(value)))),
                    Err(_) => None,
                };

                if let Some(left) = left.clone() {
                    queue.push_back(left.clone());
                }
                if let Some(right) = right.clone() {
                    queue.push_back(right.clone());
                }
                node.borrow_mut().left = left.clone();
                node.borrow_mut().right = right.clone();
            }

            Some(root)
        }
    }
}
pub mod bfs {
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
    #[allow(dead_code)]
    struct Codec {}
    #[allow(dead_code)]
    impl Codec {
        fn new() -> Self {
            Self {}
        }

        fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
            let mut result: String = String::new();
            let mut queue: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
            queue.push(root);

            while let Some(n) = queue.pop() {
                let Some(node) = n else {
                    result.push_str(&format!("n,"));
                    continue;
                };
                let x = node.borrow();
                result.push_str(&format!("{},", x.val));
                queue.push(x.right.clone());
                queue.push(x.left.clone());
            }

            result
        }

        fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
            if data.is_empty() {
                return None;
            }
            let data: &[u8] = data.as_bytes();

            if data[0] == b'n' {
                return None;
            }

            let mut queue: VecDeque<Option<i32>> = Self::to_queue(data);

            Self::bfs(&mut queue)
        }

        fn to_queue(data: &[u8]) -> VecDeque<Option<i32>> {
            let mut result: VecDeque<Option<i32>> = VecDeque::with_capacity(10_000);
            if data[0] == b'n' {
                return result;
            }

            let mut start: usize = 0;

            while start < data.len() {
                if data[start] == b'n' {
                    start += 2;
                    result.push_back(None);
                    continue;
                }
                let mut end = start;
                while end < data.len() && data[end] != b',' {
                    end += 1;
                }

                let value: i32 = std::str::from_utf8(&data[start..end])
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                result.push_back(Some(value));
                start = end + 1;
            }

            result
        }

        fn bfs(queue: &mut VecDeque<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
            let Some(value) = queue.pop_front() else {
                return None;
            };
            let Some(value) = value else {
                return None;
            };

            Some(Rc::new(RefCell::new(TreeNode {
                val: value,
                left: Self::bfs(queue),
                right: Self::bfs(queue),
            })))
        }
    }
}
