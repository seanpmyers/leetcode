pub mod map2 {
    pub struct Solution;
    use std::collections::HashMap;
    impl Solution {
        pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
            let mut map: HashMap<[u8; 26usize], Vec<String>> = HashMap::new();

            let mut count: [u8; 26usize] = [0u8; 26usize];

            for x in strs.into_iter() {
                count.fill(0u8);
                let bytes = x.as_bytes();
                Self::to_count(bytes, &mut count);
                map.entry(count.clone()).or_default().push(x);
            }

            map.into_values().collect()
        }

        pub fn to_count(bytes: &[u8], count: &mut [u8; 26usize]) {
            for i in 0..bytes.len() {
                let x: u8 = bytes[i] - b'a';
                count[x as usize] += 1;
            }
        }
    }
}
pub mod map {
    // #arrays #hashing
    pub struct Solution {}
    use std::collections::HashMap;
    impl Solution {
        pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
            let mut map: HashMap<[i32; 26], Vec<String>> = HashMap::new();
            for str in strs.into_iter() {
                let count = to_count(&str);
                map.entry(count).or_insert(vec![]).push(str);
            }
            map.into_values().collect::<Vec<Vec<String>>>()
        }
    }

    pub fn to_count(s: &String) -> [i32; 26] {
        let mut count: [i32; 26] = [0i32; 26];
        let a: usize = 'a' as usize;
        for c in s.chars() {
            count[c as usize - a] += 1;
        }
        count
    }
}
