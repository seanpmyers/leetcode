pub mod hashmap {
    pub struct Solution;
    use std::collections::HashMap;
    impl Solution {
        pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
            let len: usize = nums.len() / 3 + 1;
            let mut result: Vec<i32> = Vec::with_capacity(4usize);
            let mut map: HashMap<i32, u16> = HashMap::with_capacity(nums.len());

            for &n in nums.iter() {
                let x = map.entry(n).or_default();
                *x += 1;
                if *x >= len as u16 && !result.iter().any(|&c| c == n) {
                    result.push(n);
                }
            }

            result
        }
    }
}
