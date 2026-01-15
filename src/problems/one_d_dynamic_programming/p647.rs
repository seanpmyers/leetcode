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
