#[allow(dead_code)]
pub mod iter {
    pub struct Solution;
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
    impl Solution {
        pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
            let mut list: Vec<i32> = Vec::new();

            while let Some(mut node) = head {
                list.push(node.val);
                head = node.next.take();
            }

            list.iter().rev().eq(list.iter())
        }
    }
}
