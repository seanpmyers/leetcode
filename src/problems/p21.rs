use std::{
    borrow::{Borrow, BorrowMut},
    io::Read,
    ops::DerefMut,
};

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
        let mut r = &mut list1;

        while list2.is_some() {
            if r.is_none() || list2.as_mut()?.val < r.as_mut()?.val {
                std::mem::swap(&mut list2, r);
            }
            r = &mut r.as_mut()?.next;
        }

        list1
    }
}
