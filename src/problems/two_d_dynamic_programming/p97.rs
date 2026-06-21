pub mod dp {
    pub struct Solution;
    impl Solution {
        pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
            if s3.len().abs_diff(s1.len() + s2.len()) > 1 {
                return false;
            }

            let x = s1.as_bytes();
            let y = s2.as_bytes();
            let z = s3.as_bytes();

            let mut dp: Vec<Vec<Option<bool>>> = vec![vec![None; y.len() + 1]; x.len() + 1];

            Self::dfs(x, y, z, &mut dp)
        }

        pub fn dfs(x: &[u8], y: &[u8], z: &[u8], dp: &mut Vec<Vec<Option<bool>>>) -> bool {
            if x.is_empty() && y.is_empty() && z.is_empty() {
                return true;
            }

            if let Some(result) = dp[x.len()][y.len()] {
                return result;
            }

            if z.is_empty() && (x.len() + y.len()) > 0 {
                return false;
            }
            let mut x_works = false;
            let mut y_works = false;
            if !x.is_empty() && x[0] == z[0] {
                x_works = Self::dfs(&x[1..x.len()], y, &z[1..z.len()], dp);
            }

            if !y.is_empty() && y[0] == z[0] {
                y_works = Self::dfs(x, &y[1..y.len()], &z[1..z.len()], dp);
            }

            dp[x.len()][y.len()] = Some(x_works || y_works);
            x_works || y_works
        }
    }
}
pub mod too_slow {
    pub struct Solution;
    impl Solution {
        pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
            if s3.len().abs_diff(s1.len() + s2.len()) > 1 {
                return false;
            }

            let x = s1.as_bytes();
            let y = s2.as_bytes();
            let z = s3.as_bytes();

            // let mut dp: Vec<bool> = vec![false; y.len()];

            Self::dfs(
                x, y, z,
                // &mut dp,
            )
        }

        pub fn dfs(
            x: &[u8],
            y: &[u8],
            z: &[u8],
            // dp: &mut Vec<i32>,
        ) -> bool {
            if x.is_empty() && y.is_empty() && z.is_empty() {
                return true;
            }

            if z.is_empty() && (x.len() + y.len()) > 0 {
                return false;
            }

            if !x.is_empty() && x[0] == z[0] {
                if Self::dfs(&x[1..x.len()], y, &z[1..z.len()]) {
                    return true;
                }
            }

            if !y.is_empty() && y[0] == z[0] {
                if Self::dfs(x, &y[1..y.len()], &z[1..z.len()]) {
                    return true;
                }
            }

            false
        }
    }
}
