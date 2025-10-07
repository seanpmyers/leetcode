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
