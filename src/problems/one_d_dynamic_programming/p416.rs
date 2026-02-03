pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        // time: O(n)
        let sum: i32 = nums.iter().sum();

        if sum % 2 != 0 {
            return false;
        }

        let n: usize = nums.len();
        let m: usize = (sum / 2) as usize;

        let mut memory: Vec<Vec<Option<bool>>> = vec![vec![None; m + 1]; n];

        Self::dfs(&nums, &mut memory, 0usize, sum / 2)
    }
    pub fn dfs(
        nums: &Vec<i32>,
        memory: &mut Vec<Vec<Option<bool>>>,
        i: usize,
        target: i32,
    ) -> bool {
        if i == nums.len() {
            return target == 0;
        }

        if target < 0 {
            return false;
        }

        if let Some(value) = memory[i][target as usize] {
            return value;
        }

        memory[i][target as usize] = Some(
            Self::dfs(nums, memory, i + 1, target - nums[i])
                || Self::dfs(nums, memory, i + 1, target),
        );

        memory[i][target as usize].unwrap()
    }
}
