pub struct Solution {}
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        const LOWER_A: u8 = b'a' as u8;
        const LOWER_Z: u8 = b'z' as u8;
        const UPPER_A: u8 = b'A' as u8;
        const UPPER_Z: u8 = b'Z' as u8;
        const ZERO: u8 = b'0' as u8;
        const NINE: u8 = b'9' as u8;
        pub fn is_valid_char(c: u8) -> bool {
            c >= LOWER_A && c <= LOWER_Z || c >= UPPER_A && c <= UPPER_Z || c >= ZERO && c <= NINE
        }
        pub fn to_lowercase(mut c: u8) -> u8 {
            if c >= UPPER_A && c <= UPPER_Z {
                c = c + (LOWER_A - UPPER_A);
            }
            c
        }

        let mut l: usize = 0;
        let mut r: usize = s.len() - 1;
        let chars = s.as_bytes();
        while l < r {
            while l < r && !is_valid_char(chars[l]) {
                l = l.saturating_add(1);
            }
            while r > l && !is_valid_char(chars[r]) {
                r = r.saturating_sub(1);
            }

            if to_lowercase(chars[l]) != to_lowercase(chars[r]) {
                return false;
            }
            l = l.saturating_add(1);
            r = r.saturating_sub(1);
        }

        true
    }
}
