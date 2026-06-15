pub mod dfs_memoized {
    pub struct Solution;
    impl Solution {
        pub fn can_partition(nums: Vec<i32>) -> bool {
            let sum = nums.iter().sum::<i32>() as usize;
            if sum % 2 != 0 {
                return false;
            }
            let target: usize = sum / 2;
            let mut dp: Vec<Vec<Option<bool>>> = vec![vec![None; target + 1]; nums.len()];

            Self::dfs(0usize, target as i32, &nums, &mut dp)
        }

        pub fn dfs(
            i: usize,
            target: i32,
            nums: &Vec<i32>,
            dp: &mut Vec<Vec<Option<bool>>>,
        ) -> bool {
            if target == 0 {
                return true;
            }

            if i == nums.len() {
                return false;
            }

            if let Some(cached) = dp[i][target as usize] {
                return cached;
            };

            let a: bool = if target >= nums[i] {
                Self::dfs(i + 1, target - nums[i], nums, dp)
            } else {
                false
            };
            let b: bool = Self::dfs(i + 1, target, nums, dp);

            dp[i][target as usize] = Some(a || b);

            return a || b;
        }
    }
}
pub mod dfs {
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
}
