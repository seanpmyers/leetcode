#[allow(dead_code)]
pub mod borrow_checker_approved {
    pub struct Solution;
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }
    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }
    impl Solution {
        pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
            let Some(head) = head else {
                return;
            };
            let mut list: Vec<Box<ListNode>> = vec![];
            let mut current = &head.as_ref().next;
            let mut len: usize = 1;
            while let Some(x) = current {
                current = &x.next;
                len += 1;
            }

            let start: usize = len / 2;
            let mut i: usize = 1;
            let mut current: &mut Box<ListNode> = head;

            while i < start && current.next.is_some() {
                i += 1;
                current = current.next.as_mut().unwrap();
            }

            let mut current: Option<Box<ListNode>> = current.next.take();
            while let Some(mut x) = current {
                current = x.next.take();
                list.push(x.to_owned());
            }
            let mut current = head;
            while let Some(mut n) = list.pop() {
                let next: Option<Box<ListNode>> = current.next.take();
                n.next = next;
                current.next = Some(n);
                current = current.next.as_mut().unwrap();
                if current.next.is_none() {
                    continue;
                }
                current = current.next.as_mut().unwrap();
            }
        }
    }
}
pub mod first {
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
}
