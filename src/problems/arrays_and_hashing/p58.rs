pub struct Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        if s.is_empty() {
            return 0i32;
        }
        let s = s.as_bytes();

        let mut count: i32 = 0;

        let mut current: usize = s.len() - 1;
        while s[current] == b' ' {
            current -= 1;
        }

        while s[current] != b' ' {
            count += 1;
            if current == 0 {
                break;
            }
            current -= 1;
        }

        count
    }
}
