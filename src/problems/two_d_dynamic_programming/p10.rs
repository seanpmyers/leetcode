pub mod dfs {
    pub struct Solution;

    impl Solution {
        pub fn is_match(s: String, p: String) -> bool {
            let s: &[u8] = s.as_bytes();
            let p: &[u8] = p.as_bytes();

            let mut dp: Vec<Vec<Option<bool>>> = vec![vec![None; 21usize]; 21usize];
            Self::dfs(0usize, 0usize, s, p, &mut dp)
        }

        pub fn dfs(
            si: usize,
            pi: usize,
            s: &[u8],
            p: &[u8],
            dp: &mut Vec<Vec<Option<bool>>>,
        ) -> bool {
            if pi == p.len() {
                return si == s.len();
            }

            if let Some(result) = dp[si][pi] {
                return result;
            }

            let same: bool = si < s.len() && (s[si] == p[pi] || p[pi] == b'.');

            let result: bool = if p.get(pi + 1).is_some_and(|x| *x == b'*') {
                Self::dfs(si, pi + 2, s, p, dp) || (same && Self::dfs(si + 1, pi, s, p, dp))
            } else {
                same && Self::dfs(si + 1, pi + 1, s, p, dp)
            };

            dp[si][pi] = Some(result);

            result
        }
    }
}
