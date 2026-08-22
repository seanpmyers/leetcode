#[allow(dead_code)]
pub mod second {
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
        pub fn merge_two_lists(
            mut list1: Option<Box<ListNode>>,
            mut list2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let mut result = ListNode::new(0);
            let mut current = &mut result.next;
            while list1.is_some() || list2.is_some() {
                let Some(mut l1) = list1 else {
                    std::mem::swap(current, &mut list2);
                    break;
                };

                let Some(mut l2) = list2 else {
                    std::mem::swap(current, &mut Some(l1));
                    break;
                };

                if l1.val <= l2.val {
                    list1 = l1.next.take();
                    std::mem::swap(current, &mut Some(l1));
                    current = &mut current.as_mut()?.next;
                    list2 = Some(l2);
                    continue;
                }
                list2 = l2.next.take();
                std::mem::swap(current, &mut Some(l2));
                current = &mut current.as_mut()?.next;
                list1 = Some(l1);
            }

            result.next
        }
    }
}
pub mod first {
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
        pub fn merge_two_lists(
            mut list1: Option<Box<ListNode>>,
            mut list2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let mut result: &mut Option<Box<ListNode>> = &mut list1;

            while list2.is_some() {
                if result.as_ref().is_none()
                    || result
                        .as_ref()
                        .is_some_and(|x| list2.as_ref().is_some_and(|y| y.val < x.val))
                {
                    std::mem::swap(result, &mut list2);
                }

                result = &mut result.as_mut()?.next;
            }

            list1
        }
    }
}
