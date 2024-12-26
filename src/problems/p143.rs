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
        use std::collections::VecDeque;
        let mut list: VecDeque<Box<ListNode>> = VecDeque::new();
        let mut next: Option<Box<ListNode>> = head.take();
        while let Some(mut node) = next {
            next = node.next.take();
            list.push_back(node);
        }

        let mut dummy_head = ListNode::new(0);
        let mut iter = &mut dummy_head;
        let mut front: bool = true;
        while !list.is_empty() {
            iter.next = if front {
                list.pop_front()
            } else {
                list.pop_back()
            };
            front = !front;
            iter = iter.next.as_mut().unwrap();
        }

        *head = dummy_head.next;
    }
}
