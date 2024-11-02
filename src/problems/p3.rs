pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0i32;
        }
        let chars: Vec<char> = s.chars().collect();
        let mut result: i32 = 1i32;
        let mut l: usize = 0;
        let mut set: HashSet<char> = HashSet::new();
        for r in 0..chars.len() {
            while set.contains(&chars[r]) {
                set.remove(&chars[l]);
                l += 1;
            }
            set.insert(chars[r]);
            result = result.max(set.len() as i32);
        }
        result.max(set.len() as i32)
    }
}
