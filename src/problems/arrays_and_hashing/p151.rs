pub struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut s = s.into_bytes();
        Self::swap(&mut s);
        let mut i: usize = 0;
        while i < s.len() {
            if s[i] == b' ' {
                i += 1;
                continue;
            }
            let mut end = i + 1;
            while end < s.len() && s[end] != b' ' {
                end += 1;
            }
            Self::swap(&mut s[i..end]);
            i = end;

            if i >= s.len() {
                continue;
            }
        }
        let mut i: usize = 0;
        while i < s.len() {
            if i == 0 && s[i] == b' ' {
                s.remove(i);
                continue;
            }
            if i > 0 && s[i - 1] == b' ' && s[i] == b' ' {
                s.remove(i);
                continue;
            }
            i += 1;
        }

        if s[s.len() - 1] == b' ' {
            s.remove(s.len() - 1);
        }

        s.into_iter().map(|x| x as char).collect::<String>()
    }

    pub fn swap(slice: &mut [u8]) {
        let middle: usize = slice.len() / 2;
        for i in 0..middle {
            let swap: u8 = slice[i];
            let y: usize = slice.len() - 1 - i;
            slice[i] = slice[y];
            slice[y] = swap;
        }
    }
}
