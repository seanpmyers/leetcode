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
pub mod recursive {
    use crate::problems::linked_list::p206::ListNode;

    pub struct Solution;
    impl Solution {
        pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let Some(mut head) = head else {
                return None;
            };
            Some(Self::recurse(head.next.take(), head))
        }

        pub fn recurse(node: Option<Box<ListNode>>, head: Box<ListNode>) -> Box<ListNode> {
            let Some(mut node) = node else {
                return head;
            };

            let next = node.next.take();
            node.next = Some(head);
            Self::recurse(next, node)
        }
    }
}

pub mod misc {
    use crate::problems::linked_list::p206::ListNode;

    pub struct Solution {}

    impl Solution {
        pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            impl_1(head)
        }
    }

    #[allow(dead_code)]
    pub fn impl_1(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut result: Option<Box<ListNode>> = None;
        while let Some(mut node) = head {
            let temp = node.as_mut();
            let next = temp.next.take();
            head = next;
            temp.next = result.take();
            result = Some(node);
        }

        result
    }

    #[allow(dead_code)]
    pub fn impl_2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut current = &mut head.as_mut()?.next.take();

        while current.is_some() {
            let mut next = current.as_mut()?.next.take();
            current = current;
            std::mem::swap(&mut head, &mut current.as_mut()?.next);
            std::mem::swap(&mut head, current);
            std::mem::swap(current, &mut next);
        }

        head
    }

    #[allow(dead_code)]
    pub fn impl_3(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = result.take();
            result = Some(node);
        }
        result
    }
}
