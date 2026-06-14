pub mod dfs_memoized {
    pub struct Solution;

    impl Solution {
        pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
            let mut pairs: Vec<(usize, usize)> = Vec::with_capacity(strs.len());
            let len: usize = strs.len();
            for s in strs.into_iter() {
                let bytes = s.as_bytes();
                let mut zeros: usize = 0;
                let mut ones: usize = 0;
                for &b in bytes.iter() {
                    if b == b'0' {
                        zeros += 1;
                    } else {
                        ones += 1;
                    }
                }
                pairs.push((zeros, ones));
            }

            let mut dp: Vec<Vec<Vec<i32>>> =
                vec![vec![vec![-1i32; n as usize + 1]; m as usize + 1]; len];

            Self::dfs(0usize, m as usize, n as usize, &pairs, &mut dp).max(0i32)
        }

        pub fn dfs(
            i: usize,
            zeros: usize,
            ones: usize,
            pairs: &Vec<(usize, usize)>,
            dp: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            if i >= pairs.len() || (zeros == 0 && ones == 0) {
                return 0i32;
            }

            if dp[i][zeros][ones] == -1i32 {
                let mut take: i32 = 0i32;
                if pairs[i].0 <= zeros && pairs[i].1 <= ones {
                    take = 1 + Self::dfs(i + 1, zeros - pairs[i].0, ones - pairs[i].1, pairs, dp);
                }
                let skip: i32 = dp[i][zeros][ones].max(Self::dfs(i + 1, zeros, ones, pairs, dp));
                dp[i][zeros][ones] = take.max(skip);
            }

            dp[i][zeros][ones]
        }
    }
}
