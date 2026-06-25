pub mod dp {
    pub struct Solution;
    impl Solution {
        pub fn longest_palindrome_subseq(s: String) -> i32 {
            if s.len() == 1 {
                return 1i32;
            }

            let s: &[u8] = s.as_bytes();
            let mut dp: Vec<Vec<i32>> = vec![vec![-1i32; s.len() + 1]; s.len() + 1];

            let mut result: i32 = 1i32;

            for i in 0..s.len() {
                let even = Self::dfs(s, i as isize, i + 1, &mut dp);
                let odd = Self::dfs(s, i as isize, i, &mut dp);
                result = result.max(odd).max(even);
            }

            result
        }

        pub fn dfs(seq: &[u8], left: isize, right: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
            if left < 0 || right == seq.len() {
                return 0i32;
            }

            let left: usize = left as usize;

            if dp[left][right] != -1i32 {
                return dp[left][right];
            }

            if seq[left] == seq[right] {
                dp[left][right] = Self::dfs(seq, left as isize - 1, right + 1, dp)
                    + match left == right {
                        true => 1,
                        false => 2,
                    };
                return dp[left][right];
            }

            dp[left][right] = Self::dfs(seq, left as isize - 1, right, dp).max(Self::dfs(
                seq,
                left as isize,
                right + 1,
                dp,
            ));

            dp[left][right]
        }
    }
}
