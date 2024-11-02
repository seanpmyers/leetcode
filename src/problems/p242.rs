pub struct Solution {}
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count: [i32; 26] = [0i32; 26];
        let a: usize = 'a' as usize;
        for (x, y) in s.chars().zip(t.chars()) {
            count[x as usize - a] += 1;
            count[y as usize - a] -= 1;
        }
        count.iter().all(|v| *v == 0)
    }
}
