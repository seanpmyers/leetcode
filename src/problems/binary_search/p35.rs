pub struct Solution;
use std::cmp::Ordering;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len();

        while left < right {
            let middle: usize = middle(left, right);
            match nums[middle].cmp(&target) {
                Ordering::Equal => return middle as i32,
                Ordering::Greater => right = middle,
                Ordering::Less => left = middle + 1,
            };
        }

        (middle(left, right)) as i32
    }
}

pub fn middle(start: usize, end: usize) -> usize {
    (start + end) / 2
}
