use std::{cell::RefCell, rc::Rc};

pub type ListNode = Rc<RefCell<Node>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    pub val: i32,
    pub next: Option<ListNode>,
    pub random: Option<ListNode>,
}

pub struct Solution;
impl Solution {
    pub fn copy_random_list(head: Option<ListNode>) -> Option<ListNode> {
        use std::collections::{HashMap, VecDeque};
        let mut result: Option<ListNode> = None;

        let mut list: VecDeque<i32> = VecDeque::new();
        let mut map: HashMap<*mut Node, ListNode> = HashMap::new();

        let mut i: usize = 0usize;
        let mut pointer = head.clone();

        let mut result_pointer: Option<ListNode> = result.clone();

        while let Some(node) = pointer {
            pointer = node.as_ref().borrow().next.clone();
            let new = Node {
                val: node.as_ref().borrow().val,
                next: None,
                random: None,
            };
            let new_node = Rc::new(RefCell::new(new));

            list.push_back(node.as_ref().borrow().val);
            map.insert(node.as_ref().as_ptr(), new_node.clone());

            i = i.saturating_add(1);

            match result_pointer {
                Some(n) => {
                    n.as_ref().borrow_mut().next = Some(new_node.clone());
                    result_pointer = n.as_ref().borrow().next.clone();
                }
                None => {
                    result = Some(new_node);
                    result_pointer = result.clone();
                }
            }
        }

        let mut pointer = head.clone();
        let mut result_pointer: Option<ListNode> = result.clone();
        while let Some(node) = pointer {
            let rp: ListNode = result_pointer.take().unwrap();

            if let Some(random) = node.as_ref().borrow().random.as_ref() {
                rp.as_ref().borrow_mut().random = map.get(&random.as_ref().as_ptr()).cloned();
            };

            result_pointer = rp.as_ref().borrow().next.clone();
            pointer = node.as_ref().borrow().next.clone();
        }

        result
    }
}
