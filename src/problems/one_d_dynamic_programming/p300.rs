pub mod quadratic {
    pub struct Solution;
    impl Solution {
        pub fn length_of_lis(nums: Vec<i32>) -> i32 {
            let mut dynamic: Vec<i32> = vec![1i32; nums.len()];
            let mut result: i32 = 1i32;

            for i in (0..nums.len()).rev() {
                dynamic[i] += Self::find(&nums, &dynamic, i + 1, nums[i]);
                result = dynamic[i].max(result);
            }

            result
        }

        pub fn find(nums: &Vec<i32>, dynamic: &Vec<i32>, start: usize, current: i32) -> i32 {
            let mut result: i32 = 0;

            for i in start..nums.len() {
                if current >= nums[i] {
                    continue;
                }

                result = result.max(dynamic[i]);
            }

            result
        }
    }
}
