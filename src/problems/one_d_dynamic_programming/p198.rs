pub struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut first: i32 = 0;
        let mut second: i32 = 0;

        for i in 0..nums.len() {
            let new: i32 = second.max(first.saturating_add(nums[i]));
            first = second;
            second = new;
        }

        first.max(second)
    }
}
