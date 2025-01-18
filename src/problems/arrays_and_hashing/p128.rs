// #arrays #hashing

pub struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut longest: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for n in nums.into_iter() {
            if !map.contains_key(&n) {
                let previous = map.get(&(n - 1)).unwrap_or(&0).clone();
                let next = map.get(&(n + 1)).unwrap_or(&0).clone();
                let count = 1 + previous + next;
                map.insert(n, count);
                map.entry(n - previous)
                    .and_modify(|v| *v = count)
                    .or_insert(count);
                map.entry(n + next)
                    .and_modify(|v| *v = count)
                    .or_insert(count);
                longest = longest.max(count);
            }
        }

        longest
    }
}
