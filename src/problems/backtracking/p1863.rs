pub struct Solution;
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        Self::dfs(&nums, 0, 0i32)
    }

    pub fn dfs(nums: &[i32], index: usize, result: i32) -> i32 {
        if index >= nums.len() {
            return result;
        }

        Self::dfs(nums, index + 1, result ^ nums[index]) + Self::dfs(nums, index + 1, result)
    }
}
