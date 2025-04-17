pub struct Solution;
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        if s.len() == 1 {
            return true;
        }
        let s: &[u8] = s.as_bytes();

        let mut l: usize = 0;
        let mut r: usize = s.len() - 1;

        while l < r {
            if s[l] != s[r] {
                return Self::is_palindrome(&s[l + 1..r + 1]) || Self::is_palindrome(&s[l..r]);
            }

            l = l.saturating_add(1);
            r = r.saturating_sub(1);
        }

        true
    }
    pub fn is_palindrome(s: &[u8]) -> bool {
        let (mut l, mut r): (usize, usize) = (0usize, s.len() - 1);

        while l < r {
            if s[l] != s[r] {
                return false;
            }
            l = l.saturating_add(1);
            r = r.saturating_sub(1);
        }

        true
    }
}
