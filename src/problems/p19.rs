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
