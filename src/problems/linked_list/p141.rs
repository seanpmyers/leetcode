// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: *mut ListNode,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: std::ptr::null_mut(),
            val,
        }
    }
}

pub struct Solution;
impl Solution {
    pub fn has_cycle(head: *mut ListNode) -> bool {
        let mut slow = head.clone();
        let mut fast = head.clone();

        while !slow.is_null() && !fast.is_null() {
            slow = unsafe { slow.read().next };
            fast = unsafe { fast.read().next };
            if fast.is_null() {
                break;
            }
            fast = unsafe { fast.read().next };

            if fast == slow {
                return true;
            }
        }

        false
    }
}
