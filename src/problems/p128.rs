// #arrays #hashing

pub struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        let mut result: i32 = 0i32;
        for n in nums.into_iter() {
            if !map.contains_key(&n) {
                let previous: Option<&i32> = map.get(&(n - 1));
                let next: Option<&i32> = map.get(&(n + 1));
                match (previous, next) {
                    (None, None) => {
                        map.insert(n, 1);
                    }
                    (None, Some(nxt)) => {
                        map.insert(n, 1 + nxt);
                    }
                    (Some(prev), None) => {
                        map.insert(n, 1 + prev);
                    }
                    (Some(prev), Some(nxt)) => {
                        map.insert(n, prev + 1 + nxt);
                    }
                }
                let previous: Option<&i32> = map.get(&(n - 1));
                let current = *map.get(&n).unwrap();
                map.entry(n - previous.unwrap_or(&0))
                    .and_modify(|v| *v = current)
                    .or_insert(current);
                let next: Option<&i32> = map.get(&(n + 1));
                map.entry(n + next.unwrap_or(&0))
                    .and_modify(|v| *v = current)
                    .or_insert(current);
                result = result.max(*map.get(&n).unwrap());
            }
        }
        result
    }
}
