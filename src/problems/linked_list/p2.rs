#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution;
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result: ListNode = ListNode::new(0i32);
        let mut current = &mut result;
        let mut carry: i32 = 0;
        pub fn digit_calc(digit: &mut i32, carry: &mut i32) {
            *carry = *digit / 10;
            *digit = *digit % 10;
        }
        while l1.is_some() || l2.is_some() {
            match (l1.clone(), l2.clone()) {
                (None, None) => {}
                (None, Some(l2_n)) => {
                    let next = l2_n.next.clone();
                    let mut digit = carry + l2_n.val;
                    digit_calc(&mut digit, &mut carry);
                    let new_node = Box::new(ListNode::new(digit));
                    current.next = Some(new_node);
                    current = current.next.as_mut().unwrap();
                    l2 = next;
                }
                (Some(l1_n), None) => {
                    let next = l1_n.next.clone();
                    let mut digit = carry + l1_n.val;
                    digit_calc(&mut digit, &mut carry);
                    let new_node = Box::new(ListNode::new(digit));
                    current.next = Some(new_node);
                    current = current.next.as_mut().unwrap();
                    l1 = next;
                }
                (Some(l1_n), Some(l2_n)) => {
                    let next_1 = l1_n.next.clone();
                    let next_2 = l2_n.next.clone();
                    let mut digit = carry + l1_n.val + l2_n.val;
                    digit_calc(&mut digit, &mut carry);
                    let new_node = Box::new(ListNode::new(digit));
                    current.next = Some(new_node);
                    current = current.next.as_mut().unwrap();
                    l1 = next_1;
                    l2 = next_2;
                }
            }
        }
        if carry != 0 {
            let mut digit = 0 + carry;
            digit_calc(&mut digit, &mut carry);
            let new_node = Box::new(ListNode::new(digit));
            current.next = Some(new_node);
            _ = current.next.as_mut().unwrap();
        }

        result.next
    }
}
