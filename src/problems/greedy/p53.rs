pub struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut result: i32 = nums[0];
        let mut current: i32 = nums[0];
        for i in 1..nums.len() {
            result = result.max(nums[i]).max(current);

            if nums[i] > (current + nums[i]) {
                current = nums[i];
                continue;
            }
            current += nums[i];
        }

        result.max(current)
    }
}
