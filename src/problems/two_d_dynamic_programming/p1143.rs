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
