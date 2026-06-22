pub mod dp_dfs {
    pub struct Solution;
    impl Solution {
        pub fn num_distinct(s: String, t: String) -> i32 {
            if s.len() < t.len() {
                return 0i32;
            }
            let x = s.as_bytes();
            let y = t.as_bytes();

            let mut dp: Vec<Vec<i32>> = vec![vec![-1i32; y.len()]; x.len()];

            Self::dfs(x, y, 0usize, 0usize, &mut dp)
        }

        pub fn dfs(x: &[u8], y: &[u8], xi: usize, yi: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
            if yi == y.len() {
                return 1i32;
            }
            if xi == x.len() {
                return 0i32;
            }

            let matches: bool = x[xi] == y[yi];

            if dp[xi][yi] != -1i32 {
                return dp[xi][yi];
            }

            let skip: i32 = Self::dfs(x, y, xi + 1, yi, dp);
            let include: i32 = match matches {
                true => Self::dfs(x, y, xi + 1, yi + 1, dp),
                false => 0i32,
            };

            dp[xi][yi] = skip + include;

            dp[xi][yi]
        }
    }
}
