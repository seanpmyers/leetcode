// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution {}
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if let Some(node) = head.as_mut() {
            use std::collections::VecDeque;
            let mut current: Option<Box<ListNode>> = node.next.take();
            let result: &mut Box<ListNode> = node;
            let mut nodes: VecDeque<Box<ListNode>> = VecDeque::new();
            let mut list: &mut ListNode = &mut result.as_mut();

            while let Some(mut n) = current {
                let temp = n.next.take();
                nodes.push_back(n);
                current = temp;
            }

            while !nodes.is_empty() {
                if let Some(back) = nodes.pop_back() {
                    list.next = Some(back);
                    list = list.next.as_mut().unwrap();
                };
                if let Some(front) = nodes.pop_front() {
                    list.next = Some(front);
                    list = list.next.as_mut().unwrap();
                };
            }

            *head = Some(result.to_owned());
        }
    }
}
