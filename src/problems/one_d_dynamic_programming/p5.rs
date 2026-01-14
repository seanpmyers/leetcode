pub mod cubic {
    pub struct Solution;
    impl Solution {
        pub fn longest_palindrome(s: String) -> String {
            if s.len() == 1 {
                return s;
            }

            let bytes = s.as_bytes();
            let mut result: Vec<u8> = vec![];

            for i in 0..bytes.len() {
                let mut l: usize = i;
                let mut r: usize = i;

                while r < bytes.len() && bytes[l] == bytes[r] {
                    let slice = &bytes[l..=r];
                    if slice.len() > result.len() && Self::is_palindrome(slice) {
                        result = slice.to_vec();
                    }
                    l = l.saturating_sub(1);
                    r = r.saturating_add(1);
                }

                let mut l: usize = i;
                let mut r: usize = i.saturating_add(1);

                while r < bytes.len() && bytes[l] == bytes[r] {
                    let slice = &bytes[l..=r];
                    if slice.len() > result.len() && Self::is_palindrome(slice) {
                        result = slice.to_vec();
                    }
                    l = l.saturating_sub(1);
                    r = r.saturating_add(1);
                }
            }

            String::from_utf8(result).unwrap_or_default()
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
