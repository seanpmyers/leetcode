pub struct Solution {}
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;

        for i in 0..nums.len() {
            let x: i32 = i as i32 + 1;
            result ^= x;
            result ^= nums[i];
        }

        result
    }
}

pub fn folding(nums: Vec<i32>) -> i32 {
    nums.iter()
        .enumerate()
        .fold(0, |result, (i, x)| result ^ x ^ (i as i32 + 1))
}
