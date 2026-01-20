pub mod dp_dfs {
    pub struct Solution;
    use std::collections::{HashMap, HashSet};
    impl Solution {
        pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
            let bytes: &[u8] = s.as_bytes();

            let set: HashSet<&[u8]> = word_dict.iter().map(|x| x.as_bytes()).collect();
            let mut memory: HashMap<usize, bool> = HashMap::with_capacity(s.len());
            memory.insert(s.len(), true);

            Self::dp_dfs(0, bytes, &set, &mut memory)
        }

        pub fn dp_dfs(
            i: usize,
            bytes: &[u8],
            word_dict: &HashSet<&[u8]>,
            memory: &mut HashMap<usize, bool>,
        ) -> bool {
            if let Some(memo) = memory.get(&i) {
                return *memo;
            }

            for word in word_dict.iter() {
                let len = word.len().saturating_add(i);
                if len <= bytes.len() && &bytes[i..len] == *word {
                    if Self::dp_dfs(len, bytes, word_dict, memory) {
                        memory.entry(i).and_modify(|x| *x = true).or_insert(true);
                        return true;
                    }
                }
            }

            memory.entry(i).and_modify(|x| *x = false).or_insert(false);
            false
        }
    }
}

pub mod first_attempt {
    pub struct Solution;
    use std::collections::HashSet;
    impl Solution {
        pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
            let bytes: &[u8] = s.as_bytes();

            let set: HashSet<&[u8]> = word_dict.iter().map(|x| x.as_bytes()).collect();

            Self::search(bytes, &set)
        }

        pub fn search(bytes: &[u8], word_dict: &HashSet<&[u8]>) -> bool {
            if bytes.is_empty() {
                return true;
            }

            for word in word_dict.iter() {
                if word.len() > bytes.len() {
                    continue;
                }
                let slice = &bytes[0..word.len()];

                if slice == *word {
                    if Self::search(&bytes[word.len()..bytes.len()], word_dict) {
                        return true;
                    }
                }
            }

            false
        }
    }
}
