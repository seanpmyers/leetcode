pub mod linear_sliding_window {
    pub struct Solution {}
    impl Solution {
        pub fn length_of_longest_substring(s: String) -> i32 {
            let s: &[u8] = s.as_bytes();
            use std::collections::HashSet;
            if s.is_empty() {
                return 0i32;
            }

            let mut result: i32 = 1i32;
            let mut set: HashSet<u8> = HashSet::with_capacity(s.len());
            let mut l: usize = 0;

            for r in 0..s.len() {
                while set.contains(&s[r]) {
                    set.remove(&s[l]);
                    l += 1;
                }
                set.insert(s[r]);
                result = result.max(set.len() as i32);
            }

            result
        }
    }
}

pub mod hash_map {
    pub struct Solution;
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes: &[u8] = s.as_bytes();
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

pub mod quadtraic_sliding_window {
    pub struct Solution;
    impl Solution {
        pub fn length_of_longest_substring(s: String) -> i32 {
            let mut result: i32 = 0i32;
            if s.len() == 1 {
                return 1i32;
            }
            use std::collections::HashSet;
            let mut set: HashSet<u8> = HashSet::with_capacity(s.len());
            let mut l: usize = 0;
            let mut r: usize = 1;
            let s: &[u8] = s.as_bytes();
            while r < s.len() && l < r {
                set.insert(s[l]);
                while r < s.len() && set.insert(s[r]) {
                    r += 1;
                }
                result = result.max((r - l) as i32);
                set.clear();
                l = l + 1;
                if r < s.len() {
                    r = l + 1;
                }
            }

            result
        }
    }
}
