pub mod dfs {
    pub struct Solution;

    impl Solution {
        pub fn max_coins(nums: Vec<i32>) -> i32 {
            let n: usize = nums.len();
            let mut dp: Vec<Vec<i32>> = vec![vec![-1i32; n]; n];
            fn dfs(l: usize, r: usize, nums: &Vec<i32>, dp: &mut Vec<Vec<i32>>) -> i32 {
                if l > r || l >= nums.len() || r >= nums.len() {
                    return 0;
                }

                if dp[l][r] != -1i32 {
                    return dp[l][r];
                }
                dp[l][r] = 0;

                for i in l..=r {
                    let coins: i32 = if l == 0 { 1 } else { nums[l - 1] }
                        * nums[i]
                        * if r == nums.len() - 1 { 1 } else { nums[r + 1] };

                    let total: i32 = coins + dfs(l, i - 1, nums, dp) + dfs(i + 1, r, nums, dp);

                    dp[l][r] = dp[l][r].max(total);
                }

                dp[l][r]
            }

            dfs(0usize, n - 1, &nums, &mut dp)
        }
    }
}
