#[allow(dead_code)]
pub mod in_place {
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
        pub fn remove_nth_from_end(
            mut head: Option<Box<ListNode>>,
            n: i32,
        ) -> Option<Box<ListNode>> {
            let mut current = &head;
            let mut count: i32 = 0;
            while let Some(node) = current {
                count += 1;
                current = &node.next;
            }

            let mut current = &mut head;
            let mut x: i32 = 0;
            let target: i32 = count - n;

            if target == 0 {
                return head?.next.take();
            }

            while x < target - 1
                && let Some(node) = current
            {
                x += 1;
                current = &mut node.next;
            }

            if let Some(x) = current {
                x.next = x.next.take().and_then(|node| node.next);
            }

            head
        }
    }
}
pub mod first {
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
        pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
            let mut pointer = head.as_ref();
            let mut count: i32 = 0;

            while let Some(node) = pointer {
                count += 1;
                pointer = node.next.as_ref();
            }

            if count == 1 {
                return None;
            }

            let mut current: i32 = 0;
            let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
            let mut pointer = dummy.as_mut();
            while let Some(node) = pointer {
                match current == count - n {
                    true => {
                        let temp = node.next.take();
                        node.next = match temp {
                            Some(mut next) => next.as_mut().next.take(),
                            None => None,
                        };
                        return dummy.unwrap().next;
                    }
                    false => pointer = node.next.as_mut(),
                }
                current += 1;
            }

            dummy.unwrap().next
        }
    }
}
