pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;
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

    pub fn hash_map(bytes: &[u8]) -> i32 {
        use std::collections::HashMap;
        if bytes.is_empty() {
            return 0i32;
        }
        let mut result: usize = 1usize;
        let mut l: usize = 0usize;
        let mut map: HashMap<u8, usize> = HashMap::with_capacity(bytes.len());
        for r in 0..bytes.len() {
            if let Some(value) = map.get(&bytes[r]) {
                l = l.max(value + 1);
            }
            map.insert(bytes[r], r);
            result = result.max(r - l + 1);
        }

        result as i32
    }
}
