pub mod manacher {
    pub struct Solution;
    impl Solution {
        pub fn count_substrings(s: String) -> i32 {
            if s.is_empty() {
                return 0i32;
            }

            let p = Self::manacher(s.as_bytes());

            p.iter().map(|x| ((x + 1) / 2) as i32).sum::<i32>()
        }

        pub fn manacher(bytes: &[u8]) -> Vec<usize> {
            let mut values: Vec<u8> = Vec::with_capacity((bytes.len() * 2) + 1);
            values.push(b'#');
            for i in 0..bytes.len() {
                values.push(bytes[i]);
                values.push(b'#');
            }
            let n: usize = values.len();
            let mut p: Vec<usize> = vec![0usize; n];

            let (mut l, mut r): (usize, usize) = (0, 0);

            for i in 0..n {
                if i < r {
                    p[i] = (r - i).min(p[l + (r - i)]);
                }

                while i + 1 + p[i] < n
                    && i >= p[i] + 1
                    && values[i - p[i] - 1] == values[i + p[i] + 1]
                {
                    p[i] += 1;
                }

                if p[i] + i > r {
                    l = i - p[i];
                    r = i + p[i];
                }
            }

            p
        }
    }
}
pub mod cubic {
    pub struct Solution;
    impl Solution {
        pub fn count_substrings(s: String) -> i32 {
            let mut result: i32 = 0;

            let bytes: &[u8] = s.as_bytes();

            for i in 0..bytes.len() {
                let mut r: usize = i;

                while r < bytes.len() {
                    if Self::is_palindrome(&bytes[i..=r]) {
                        result = result.saturating_add(1);
                    }

                    r = r.saturating_add(1);
                }
            }

            result
        }

        pub fn is_palindrome(bytes: &[u8]) -> bool {
            for i in 0..bytes.len() {
                if bytes[i] != bytes[bytes.len().saturating_sub(1).saturating_sub(i)] {
                    return false;
                }
            }

            true
        }
    }
}
