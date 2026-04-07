pub mod iterative {
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
}

pub mod dfs_memory {
    pub struct Solution;
    impl Solution {
        pub fn rob(nums: Vec<i32>) -> i32 {
            let mut dp: Vec<i32> = vec![-1; nums.len()];
            Self::dfs(0, &nums, &mut dp)
        }

        pub fn dfs(i: usize, nums: &Vec<i32>, dp: &mut Vec<i32>) -> i32 {
            if i >= nums.len() {
                return 0;
            }

            if dp[i] > -1 {
                return dp[i];
            }

            dp[i] = Self::dfs(i + 1, nums, dp).max(Self::dfs(i + 2, nums, dp) + nums[i]);
            dp[i]
        }
    }
}

pub mod recursive_dfs {
    pub struct Solution;
    impl Solution {
        pub fn rob(nums: Vec<i32>) -> i32 {
            Self::dfs(0usize, &nums)
        }

        pub fn dfs(i: usize, nums: &Vec<i32>) -> i32 {
            if i >= nums.len() {
                return 0i32;
            }

            Self::dfs(i + 1, nums).max(Self::dfs(i + 2, nums) + nums[i])
        }
    }
}
