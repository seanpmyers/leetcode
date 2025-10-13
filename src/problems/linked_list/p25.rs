#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution;
impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        use std::collections::VecDeque;
        let k: usize = k as usize;

        let mut result: Option<Box<ListNode>> = None;

        let mut list: VecDeque<Box<ListNode>> = VecDeque::new();
        let mut k_group: VecDeque<Box<ListNode>> = VecDeque::new();

        while let Some(mut h) = head.take() {
            head = h.next.take();
            k_group.push_front(h);
            if k_group.len() == k {
                list.append(&mut k_group);
            }
        }

        if !k_group.is_empty() {
            k_group = k_group.into_iter().rev().collect();
            list.append(&mut k_group);
        }

        while let Some(mut node) = list.pop_back() {
            node.next = result.take();
            result = Some(node);
        }

        result
    }
}
