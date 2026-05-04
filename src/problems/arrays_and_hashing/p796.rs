pub struct Solution;
impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let s = s.as_bytes();
        let g = goal.as_bytes();

        if s.len() != g.len() {
            return false;
        }

        let mut current: usize = 0;

        while current < s.len() {
            while g[0] != s[current] {
                current += 1;
                if current >= s.len() {
                    return false;
                }
            }
            for i in 0..s.len() {
                let index = (current + i) % s.len();
                if g[i] != s[index] {
                    break;
                }

                if i == s.len() - 1 {
                    return true;
                }
            }
            current += 1;
        }

        false
    }
}
