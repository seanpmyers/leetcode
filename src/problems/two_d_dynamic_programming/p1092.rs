pub mod too_much_memory {
    pub struct Solution;
    impl Solution {
        pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
            if str1 == str2 {
                return str1;
            }

            let x = str1.as_bytes();
            let y = str2.as_bytes();

            let mut dp: Vec<Vec<Option<Vec<u8>>>> = vec![vec![None; y.len() + 1]; x.len() + 1];

            let result: Vec<u8> = Self::dfs(x, y, 0usize, 0usize, &mut dp);

            result.into_iter().rev().map(|x| x as char).collect()
        }

        pub fn dfs(
            x: &[u8],
            y: &[u8],
            xi: usize,
            yi: usize,
            dp: &mut Vec<Vec<Option<Vec<u8>>>>,
        ) -> Vec<u8> {
            if let Some(sequence) = dp[xi][yi].clone() {
                return sequence;
            }

            if xi == x.len() {
                let mut remaining: Vec<u8> = y[yi..].iter().rev().cloned().collect();
                dp[xi][yi] = Some(remaining.clone());
                return remaining;
            }

            if yi == y.len() {
                let mut remaining: Vec<u8> = x[xi..].iter().rev().cloned().collect();
                dp[xi][yi] = Some(remaining.clone());
                return remaining;
            }

            if x[xi] == y[yi] {
                let mut sequence = Self::dfs(x, y, xi + 1, yi + 1, dp);
                sequence.push(x[xi]);
                dp[xi][yi] = Some(sequence.clone());
                return sequence;
            }

            let mut take_x = Self::dfs(x, y, xi + 1, yi, dp);
            take_x.push(x[xi]);
            let mut take_y = Self::dfs(x, y, xi, yi + 1, dp);
            take_y.push(y[yi]);
            let result = match take_x.len() < take_y.len() {
                true => take_x,
                false => take_y,
            };
            dp[xi][yi] = Some(result.clone());
            result
        }
    }
}
