pub struct Solution;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m.min(n) == 1 {
            return 1;
        }
        let m: usize = m as usize;
        let n: usize = n as usize;

        let mut dp: Vec<Vec<i32>> = vec![vec![1; n]; m];

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }

        dp[m - 1][n - 1]
    }
}
