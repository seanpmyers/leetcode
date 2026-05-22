pub mod first_attempt {
    pub struct Solution;
    use std::collections::HashSet;
    impl Solution {
        pub fn longest_common_prefix(strs: Vec<String>) -> String {
            if strs.is_empty() {
                return String::new();
            }

            let min: Vec<u8> = strs[0].as_bytes().to_vec();
            let mut set: HashSet<&[u8]> = HashSet::new();
            for i in 0..min.len() {
                set.insert(&min[0..=i]);
            }

            let mut result: Vec<u8> = vec![];

            for s in strs.into_iter() {
                let s = s.as_bytes();
                let mut count: i32 = -1;
                for i in 0..s.len() {
                    if !set.contains(&&s[0..=i]) {
                        break;
                    }
                    count += 1;
                }
                if count < 0 {
                    return String::new();
                }
                let count = count as usize;
                if result.is_empty() {
                    result = s[0..=count].to_vec();
                    continue;
                }
                if result.len() > count {
                    result = s[0..=count].to_vec();
                }
            }

            result.into_iter().map(|x| x as char).collect::<String>()
        }
    }
}
