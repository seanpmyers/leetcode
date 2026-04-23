pub mod optimal {
    pub struct Solution;
    // Definition for singly-linked list.
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }
    impl Solution {
        pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut fast = &head;
            let mut slow = &head;

            while let Some(f) = fast {
                let Some(next_f) = &f.next else {
                    break;
                };
                fast = &next_f.next;
                slow = &slow.as_ref().unwrap().next;
            }

            slow.clone()
        }
    }
}

pub mod first {
    pub struct Solution;
    // Definition for singly-linked list.
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }
    impl Solution {
        pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut fast = head.clone();
            let mut slow = head;
            let mut count = 1;

            while let Some(mut f_n) = fast {
                let fast_next_exists: bool = f_n
                    .as_ref()
                    .next
                    .as_ref()
                    .is_some_and(|n| n.as_ref().next.as_ref().is_some());

                if fast_next_exists {
                    fast = f_n.next.unwrap().next.take();
                    count += 2;
                    slow = slow.unwrap().next.take();
                } else {
                    fast = f_n.next.take();
                    count += 1;
                }
            }

            if count % 2 == 1 {
                slow = slow.unwrap().next.take();
            }

            slow
        }
    }
}
