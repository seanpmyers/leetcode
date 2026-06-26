pub mod two_loops {
    pub struct Solution;
    impl Solution {
        pub fn longest_palindrome(s: String) -> String {
            if s.len() == 1 {
                return s;
            }

            let s: &[u8] = s.as_bytes();
            let mut l: usize = 0;
            let mut r: usize = 0;

            for i in 0..s.len() {
                let mut x: usize = i;
                let mut y: usize = i;
                while s[x] == s[y] {
                    if y - x > r - l {
                        l = x;
                        r = y;
                    }
                    if x.checked_sub(1).is_none() {
                        break;
                    }
                    if y + 1 >= s.len() {
                        break;
                    }
                    x -= 1;
                    y += 1;
                }

                x = i;
                y = i + 1;
                while y < s.len() && s[x] == s[y] {
                    if y.abs_diff(x) > l.abs_diff(r) {
                        l = x;
                        r = y;
                    }

                    if x.checked_sub(1).is_none() {
                        break;
                    }
                    if y + 1 >= s.len() {
                        break;
                    }

                    x -= 1;
                    y += 1;
                }
            }

            s[l..=r].iter().map(|x| *x as char).collect()
        }
    }
}
pub mod manacher {
    pub struct Solution;
    impl Solution {
        pub fn longest_palindrome(s: String) -> String {
            if s.is_empty() {
                return s;
            }

            Self::manacher(s.as_bytes())
        }

        pub fn manacher(bytes: &[u8]) -> String {
            let mut t: Vec<u8> = Vec::with_capacity(bytes.len() * 2 + 1);
            t.push(b'#');
            for i in 0..bytes.len() {
                t.push(bytes[i]);
                t.push(b'#');
            }

            let n: usize = t.len();
            let mut p: Vec<usize> = vec![0usize; n];
            let (mut l, mut r): (usize, usize) = (0, 0);
            let (mut max, mut max_l, mut max_r): (usize, usize, usize) = (0, 0, 0);
            for i in 0..n {
                if i < r {
                    p[i] = (r - i).min(p[l + (r - i)]);
                }

                while i + p[i] + 1 < n && i >= p[i] + 1 && t[i + p[i] + 1] == t[i - p[i] - 1] {
                    p[i] += 1;
                }

                if i + p[i] > r {
                    l = i - p[i];
                    r = i + p[i];
                }
                if p[i] > max {
                    max = p[i];
                    max_l = l;
                    max_r = r;
                }
            }

            String::from_utf8(
                (&t[max_l..max_r])
                    .iter()
                    .filter(|x| **x != b'#')
                    .map(|x| *x)
                    .collect::<Vec<u8>>(),
            )
            .unwrap()
        }
    }
}
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
