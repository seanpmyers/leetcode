pub mod optimal {
    pub struct Solution;
    impl Solution {
        pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
            if text1 == text2 {
                return text2.len() as i32;
            }

            let (x, y): (&[u8], &[u8]) = match text2.len() > text1.len() {
                true => (text2.as_bytes(), text1.as_bytes()),
                false => (text1.as_bytes(), text2.as_bytes()),
            };

            let mut dp: Vec<i32> = vec![0i32; y.len() + 1];

            for outer in (0..x.len()).rev() {
                let mut previous: i32 = 0i32;
                for inner in (0..y.len()).rev() {
                    let temp: i32 = dp[inner];
                    match x[outer] == y[inner] {
                        true => dp[inner] = 1 + previous,
                        false => dp[inner] = dp[inner].max(dp[inner + 1]),
                    };
                    previous = temp;
                }
            }

            dp[0]
        }
    }
}

pub mod iterative_bottom_up {
    pub struct Solution;
    impl Solution {
        pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
            let mut t1: &[u8] = text1.as_bytes();
            let mut t2: &[u8] = text2.as_bytes();
            if t1.len() < t2.len() {
                std::mem::swap(&mut t1, &mut t2);
            }

            let mut dp = vec![vec![0i32; t2.len() + 1]; t1.len() + 1];

            for r in (0..t1.len()).rev() {
                for c in (0..t2.len()).rev() {
                    dp[r][c] = match t1[r] == t2[c] {
                        true => 1 + dp[r + 1][c + 1],
                        false => dp[r + 1][c].max(dp[r][c + 1]),
                    };
                }
            }

            dp[0][0]
        }
    }
}

pub mod recursive_dfs_dp {
    pub struct Solution;
    impl Solution {
        pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
            let t1: &[u8] = text1.as_bytes();
            let t2: &[u8] = text2.as_bytes();

            let mut dp: Vec<Vec<i32>> = vec![vec![-1i32; t2.len()]; t1.len()];

            Self::dfs(0, 0, t1, t2, &mut dp)
        }

        pub fn dfs(x: usize, y: usize, t1: &[u8], t2: &[u8], dp: &mut Vec<Vec<i32>>) -> i32 {
            if x >= t1.len() || y >= t2.len() {
                return 0i32;
            }

            if dp[x][y] != -1 {
                return dp[x][y];
            }

            if t1[x] == t2[y] {
                dp[x][y] = 1 + Self::dfs(x + 1, y + 1, t1, t2, dp);
            } else {
                dp[x][y] = Self::dfs(x + 1, y, t1, t2, dp).max(Self::dfs(x, y + 1, t1, t2, dp));
            }

            dp[x][y]
        }
    }
}

pub mod recursive_dfs_dp_reverse {
    pub struct Solution;
    impl Solution {
        pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
            let t1: &[u8] = text1.as_bytes();
            let t2: &[u8] = text2.as_bytes();

            let mut dp: Vec<Vec<i32>> = vec![vec![-1i32; t2.len()]; t1.len()];

            Self::dfs(t1.len() - 1, t2.len() - 1, t1, t2, &mut dp)
        }

        pub fn dfs(x: usize, y: usize, t1: &[u8], t2: &[u8], dp: &mut Vec<Vec<i32>>) -> i32 {
            if dp[x][y] != -1 {
                return dp[x][y];
            }

            if x == 0 && y == 0 {
                return if t1[x] == t2[y] { 1 } else { 0 };
            }

            if t1[x] == t2[y] {
                dp[x][y] = match (x.checked_sub(1), y.checked_sub(1)) {
                    (Some(dx), Some(dy)) => 1 + Self::dfs(dx, dy, t1, t2, dp),
                    (_, _) => 1i32,
                };
                return dp[x][y];
            }

            if let Some(dx) = x.checked_sub(1) {
                dp[x][y] = dp[x][y].max(Self::dfs(dx, y, t1, t2, dp));
            }

            if let Some(dy) = y.checked_sub(1) {
                dp[x][y] = dp[x][y].max(Self::dfs(x, dy, t1, t2, dp));
            }

            dp[x][y]
        }
    }
}

pub mod recursive_too_slow {
    pub struct Solution;
    impl Solution {
        pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
            let t1: &[u8] = text1.as_bytes();
            let t2: &[u8] = text2.as_bytes();

            Self::dfs(0, 0, t1, t2)
        }

        pub fn dfs(x: usize, y: usize, t1: &[u8], t2: &[u8]) -> i32 {
            if x >= t1.len() || y >= t2.len() {
                return 0i32;
            }

            if t1[x] == t2[y] {
                return 1 + Self::dfs(x + 1, y + 1, t1, t2);
            }

            Self::dfs(x + 1, y, t1, t2).max(Self::dfs(x, y + 1, t1, t2))
        }
    }
}

pub mod first_attempt {
    pub struct Solution;
    use std::collections::HashMap;
    impl Solution {
        pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
            if text1 == text2 {
                return text2.len() as i32;
            }

            let (x, y): (&[u8], &[u8]) = match text2.len() > text1.len() {
                true => (text2.as_bytes(), text1.as_bytes()),
                false => (text1.as_bytes(), text2.as_bytes()),
            };

            let mut map: HashMap<u8, Vec<usize>> = HashMap::with_capacity(x.len());
            for i in 0..x.len() {
                map.entry(x[i]).or_default().push(i);
            }

            let mut dp: Vec<Vec<usize>> = vec![];
            let mut result: i32 = 0i32;
            for i in 0..y.len() {
                let Some(x_i) = map.get(&y[i]) else { continue };
                for j in 0..dp.len() {
                    // println!("");
                    let last: usize = dp[j][dp[j].len() - 1];
                    for location in x_i.iter() {
                        if *location <= last {
                            continue;
                        }
                        dp[j].push(*location);
                        result = result.max(dp[j].len() as i32);
                        break;
                    }
                    // println!("{i} {:?} {:?}", y[i] as char, dp[j].iter().map(|m| x[*m] as char).collect::<Vec<char>>());
                }

                dp.push(vec![x_i[0]]);
                result = result.max(1);
            }

            result
        }
    }
}
