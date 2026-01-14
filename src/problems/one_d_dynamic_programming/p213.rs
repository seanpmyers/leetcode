pub struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        Self::helper(&nums[1..nums.len()])
            .max(Self::helper(&nums[0..nums.len().saturating_sub(1)]))
            .max(nums[0])
    }

    pub fn helper(nums: &[i32]) -> i32 {
        let mut first: i32 = 0i32;
        let mut second: i32 = 0i32;

        for num in nums.iter() {
            let current: i32 = second.saturating_add(*num).max(first);
            second = first;
            first = current;
        }

        first.max(second)
    }
}
