use std::borrow::{Borrow, BorrowMut};

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
pub struct Solution {}
impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut fast = dummy.as_ref().next.as_ref();
        let mut slow = Some(&dummy);
        let mut count: i32 = 1;
        let mut current: i32 = 0;
        while count > current {
            match fast {
                Some(node) => {
                    if node.next.is_some() {
                        fast = node.next.as_ref();
                        count += 1;
                        if let Some(second) = fast {
                            count += 1;
                            fast = second.next.as_ref();
                            if let Some(slow_next) = slow.as_ref() {
                                slow = slow_next.next.as_ref();
                                current += 1;
                            }
                        }
                    }
                }
                None => {
                    while count > current {
                        if current == count - n {
                            if let Some(mut s) = slow {
                                // let temp = s.borrow_mut().next.take();
                            }
                        }
                    }
                }
            }
        }
        dummy.next.take()
    }
}
