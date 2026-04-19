pub struct Solution;
use std::cmp::Ordering;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = height.len() - 1;
        let mut result: i32 = 0;

        while l < r {
            result = result.max(height[l].min(height[r]) * (l.abs_diff(r) as i32));

            match height[l].cmp(&height[r]) {
                Ordering::Equal | Ordering::Less => l += 1,
                Ordering::Greater => r -= 1,
            };
        }

        result
    }
}
