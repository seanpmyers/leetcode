pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let pattern: &[u8] = pattern.as_bytes();
        let mut start: usize = 0;
        let s: &[u8] = s.as_bytes();
        let mut map: HashMap<u8, Vec<u8>> = HashMap::with_capacity(pattern.len());
        let mut word_map: HashMap<Vec<u8>, u8> = HashMap::with_capacity(pattern.len());
        for i in 0..pattern.len() {
            if start >= s.len() {
                return false;
            }

            let mut end: usize = start;
            while end < s.len() && s[end] != b' ' {
                end += 1;
            }
            let word: Vec<u8> = s[start..end].to_vec();
            start = end;
            while start < s.len() && s[start] == b' ' {
                start += 1;
            }
            if map.get(&pattern[i]).is_some_and(|x| *x != word) {
                return false;
            }
            if word_map.get(&word).is_some_and(|x| *x != pattern[i]) {
                return false;
            }
            map.insert(pattern[i], word.clone());
            word_map.insert(word, pattern[i]);
        }

        if start < s.len() {
            return false;
        }

        true
    }
}
