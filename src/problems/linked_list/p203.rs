#[allow(dead_code)]
pub mod no_allocation {
    // Definition for singly-linked list.
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

    pub struct Solution;
    impl Solution {
        pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
            while head.as_ref().is_some_and(|node| node.val == val) {
                head = head.unwrap().next;
            }

            if head.is_none() {
                return None;
            }
            let mut head = head.unwrap();
            let mut current = Some(&mut head);

            while let Some(node) = current {
                if node.next.as_ref().is_some_and(|n| n.val != val) {
                    current = node.next.as_mut();
                    continue;
                }
                let mut next = node.as_mut().next.take();
                while next.as_ref().is_some_and(|x| x.val == val) {
                    next = next.unwrap().next;
                }
                node.next = next;
                current = node.next.as_mut();
            }

            Some(head)
        }
    }
}
