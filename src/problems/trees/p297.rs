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
