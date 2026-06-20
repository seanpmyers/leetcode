pub mod dfs_memoization {
    pub struct Solution;
    impl Solution {
        pub fn min_distance(word1: String, word2: String) -> i32 {
            if word1.is_empty() || word2.is_empty() {
                return word1.len().max(word2.len()) as i32;
            }
            let (x, y) = (word1.as_bytes(), word2.as_bytes());
            let length: usize = x.len().max(y.len());
            let mut dp: Vec<Vec<i32>> = vec![vec![-1i32; length]; length];

            Self::dfs(x, y, 0usize, 0usize, &mut dp)
        }

        pub fn dfs(x: &[u8], y: &[u8], xi: usize, yi: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
            if xi == x.len() {
                return (y.len() - yi) as i32;
            }

            if yi == y.len() {
                return (x.len() - xi) as i32;
            }

            if dp[xi][yi] != -1i32 {
                return dp[xi][yi];
            }

            if x[xi] == y[yi] {
                dp[xi][yi] = Self::dfs(x, y, xi + 1, yi + 1, dp);
            } else {
                let replace = Self::dfs(x, y, xi + 1, yi + 1, dp);
                let delete = Self::dfs(x, y, xi + 1, yi, dp);
                let insert = Self::dfs(x, y, xi, yi + 1, dp);
                dp[xi][yi] = 1 + replace.min(insert).min(delete);
            }

            dp[xi][yi]
        }
    }
}
