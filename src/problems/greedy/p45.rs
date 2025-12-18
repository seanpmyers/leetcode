pub mod exponential {
    pub struct Solution;
    impl Solution {
        pub fn jump(nums: Vec<i32>) -> i32 {
            let mut result: i32 = 0;
            let mut goal: usize = nums.len() - 1;

            while goal != 0 {
                for i in 0..goal {
                    if i.saturating_add(nums[i] as usize) == goal {
                        goal = i;
                        result = result.saturating_add(1);
                        break;
                    }
                }
            }

            result
        }
    }
}
