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
