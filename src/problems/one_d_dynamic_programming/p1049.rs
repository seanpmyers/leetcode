pub mod dp {
    pub struct Solution;
    impl Solution {
        pub fn last_stone_weight_ii(mut stones: Vec<i32>) -> i32 {
            if stones.len() == 1 {
                return stones[0];
            }
            let sum: i32 = stones.iter().sum();
            let target: i32 = (sum + 1) / 2;
            let mut dp: Vec<Vec<i32>> = vec![vec![-1i32; target as usize + 1usize]; stones.len()];

            Self::dfs(0, 0, &stones, sum, target, &mut dp)
        }

        pub fn dfs(
            i: usize,
            total: i32,
            stones: &Vec<i32>,
            sum: i32,
            target: i32,
            dp: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if total >= target || i == stones.len() {
                return (total - (sum - total)).abs();
            }

            if dp[i][total as usize] == -1i32 {
                dp[i][total as usize] = Self::dfs(i + 1, total, stones, sum, target, dp)
                    .min(Self::dfs(i + 1, total + stones[i], stones, sum, target, dp));
            }

            dp[i][total as usize]
        }
    }
}
