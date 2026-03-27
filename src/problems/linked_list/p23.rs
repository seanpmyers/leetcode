pub mod merge_sort {
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

    pub struct Solution;
    impl Solution {
        pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
            if lists.len() <= 1 {
                return lists.pop().unwrap_or(None);
            }

            let middle = lists.split_off(lists.len() / 2);
            let left = Self::merge_k_lists(lists);
            let right = Self::merge_k_lists(middle);

            Self::merge(left, right)
        }

        pub fn merge(
            mut list1: Option<Box<ListNode>>,
            mut list2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let mut current = &mut list1;

            while list2.as_ref().is_some() {
                if current.as_ref().is_none()
                    || current.as_ref().is_some_and(|one| {
                        list2
                            .as_ref()
                            .is_some_and(|two| one.as_ref().val > two.as_ref().val)
                    })
                {
                    std::mem::swap(current, &mut list2);
                }

                current = &mut current.as_mut()?.next;
            }

            list1
        }
    }
}

pub mod linear {
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

    pub struct Solution;
    impl Solution {
        pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
            if lists.is_empty() {
                return None;
            }

            let mut result: ListNode = ListNode::new(0i32);
            let current = &mut result.next;
            while let Some(list) = lists.pop() {
                *current = Self::merge(current.take(), list);
            }

            result.next.take()
        }

        pub fn merge(
            mut list1: Option<Box<ListNode>>,
            mut list2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let mut current = &mut list1;

            while list2.as_ref().is_some() {
                if current.as_ref().is_none()
                    || current.as_ref().is_some_and(|one| {
                        list2
                            .as_ref()
                            .is_some_and(|two| one.as_ref().val > two.as_ref().val)
                    })
                {
                    std::mem::swap(current, &mut list2);
                }

                current = &mut current.as_mut()?.next;
            }

            list1
        }
    }
}
