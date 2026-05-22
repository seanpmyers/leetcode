pub mod hashmap {
    pub struct Solution;

    use std::collections::HashMap;
    impl Solution {
        pub fn majority_element(nums: Vec<i32>) -> i32 {
            let mut result: i32 = nums[0];
            let mut count: i32 = 1;
            let mut map: HashMap<i32, i32> = HashMap::new();
            map.insert(result, count);
            for i in 1..nums.len() {
                map.entry(nums[i])
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                let current = *map.get(&nums[i]).unwrap();
                if count > current {
                    continue;
                }
                result = nums[i];
                count = current;
            }

            result
        }
    }
}
