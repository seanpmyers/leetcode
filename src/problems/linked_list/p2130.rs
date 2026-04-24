// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;
impl Solution {
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let mut list: Vec<i32> = vec![];
        let mut result: i32 = 0;

        while let Some(mut l) = head {
            list.push(l.as_ref().val);
            head = l.as_mut().next.take();
        }

        for i in 0..(list.len() / 2) {
            result = result.max(list[i] + list[Self::twin(i, list.len())]);
        }

        result
    }

    pub fn twin(i: usize, n: usize) -> usize {
        n - 1 - i
    }
}
