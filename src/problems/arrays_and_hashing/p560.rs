pub mod linear {
    pub struct Solution;
    use std::collections::HashMap;
    impl Solution {
        pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
            let mut map: HashMap<i32, i32> = HashMap::new();
            let mut result: i32 = 0;
            let mut current: i32 = 0;
            map.insert(0, 1);
            for i in 0..nums.len() {
                current += nums[i];
                let diff: i32 = current - k;
                result += *map.get(&diff).unwrap_or(&0);
                map.entry(current).and_modify(|x| *x += 1).or_insert(1);
            }

            result
        }
    }
}

pub mod quadtratic {
    pub struct Solution;
    impl Solution {
        pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
            let mut prefix: Vec<i32> = vec![0; nums.len()];
            prefix[0] = nums[0];
            for i in 1..nums.len() {
                prefix[i] = prefix[i - 1] + nums[i];
            }

            let mut result: i32 = 0;

            for l in 0..nums.len() {
                for r in l..nums.len() {
                    let mut x: i32 = prefix[r];
                    if let Some(left) = l.checked_sub(1) {
                        x -= prefix[left];
                    }
                    if x == k {
                        result += 1;
                    }
                }
            }

            result
        }
    }
}
