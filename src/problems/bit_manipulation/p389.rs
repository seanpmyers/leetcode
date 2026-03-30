pub mod xor {
    pub struct Solution;
    impl Solution {
        pub fn find_the_difference(s: String, t: String) -> char {
            let mut t_iter = t.bytes();
            let mut result: u8 = s
                .bytes()
                .zip(t_iter.by_ref())
                .fold(0, |acc, (x, y)| acc ^ x ^ y);

            result ^= t_iter.next().unwrap();

            result as char
        }
    }
}

pub mod char_count {
    pub struct Solution;
    impl Solution {
        pub fn find_the_difference(s: String, t: String) -> char {
            let s = s.as_bytes();
            let t = t.as_bytes();
            let mut count: [i16; 26usize] = [0i16; 26usize];

            for i in 0..s.len() {
                count[(s[i] - b'a') as usize] += 1;
                count[(t[i] - b'a') as usize] -= 1;
            }

            count[(t[s.len()] - b'a') as usize] -= 1;
            for i in 0..26 {
                if count[i] == -1 {
                    return (i as u8 + b'a') as char;
                }
            }
            '!'
        }
    }
}
