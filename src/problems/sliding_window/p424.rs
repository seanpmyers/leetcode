pub mod optimal {
    pub struct Solution {}
    impl Solution {
        pub fn character_replacement(s: String, k: i32) -> i32 {
            use std::collections::HashMap;
            let mut max: i32 = 0i32;
            let bytes: &[u8] = s.as_bytes();
            let mut l: usize = 0;
            let mut max_f: i32 = 0;
            let mut map: HashMap<u8, i32> = HashMap::with_capacity(26usize);
            for r in 0..bytes.len() {
                let entry: &mut i32 = map.entry(bytes[r]).and_modify(|x| *x += 1).or_insert(1i32);
                max_f = max_f.max(*entry);
                while (r as i32 - l as i32 + 1i32) - max_f > k {
                    map.entry(bytes[l]).and_modify(|x| *x -= 1);
                    l = l.saturating_add(1);
                }
                max = max.max(r as i32 - l as i32 + 1i32);
            }

            max
        }
    }
}

pub mod sliding_window {
    pub struct Solution;
    use std::collections::HashSet;
    impl Solution {
        pub fn character_replacement(s: String, k: i32) -> i32 {
            let mut result: i32 = 1;
            let s: &[u8] = s.as_bytes();

            let letters: HashSet<u8> = s.to_vec().into_iter().collect();

            for &letter in &letters {
                let mut count: i32 = 0;
                let mut l: usize = 0;
                for r in 0..s.len() {
                    if s[r] == letter {
                        count += 1;
                    }
                    while (r - l + 1) as i32 - count > k {
                        if s[l] == letter {
                            count -= 1;
                        }
                        l += 1;
                    }
                    result = result.max(l.abs_diff(r + 1) as i32);
                }
            }

            result
        }
    }
}
