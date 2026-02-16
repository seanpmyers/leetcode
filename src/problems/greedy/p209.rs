pub struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.iter().sum::<i32>() < target {
            return 0i32;
        }

        let mut result: usize = nums.len();
        let mut current: i32 = 0;
        let mut start: usize = 0;

        for i in 0..nums.len() {
            current += nums[i];

            if current >= target {
                result = result.min(i - start + 1);
                while current - nums[start] >= target {
                    current -= nums[start];
                    start += 1;
                }
                result = result.min(i - start + 1);
            }
        }

        result as i32
    }
}
