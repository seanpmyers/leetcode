/// time complexity: O(n) linear -- worst case checking each number once
/// space complexity: O(1) constant -- storing only some additional integers
pub mod greedy {
    pub struct Solution;
    impl Solution {
        pub fn jump(nums: Vec<i32>) -> i32 {
            let mut result: i32 = 0;
            let goal: usize = nums.len() - 1;
            let mut left: usize = 0usize;
            let mut right: usize = 0usize;

            while right < goal {
                let mut max = 0;
                for i in left..=right {
                    max = max.max(i.saturating_add(nums[i] as usize));
                }

                left = right.saturating_add(1);
                right = max;
                result = result.saturating_add(1);
            }

            result
        }
    }
}

/// time complexity: O(n^2) quadratic -- checking the same numbers multiple times
/// space complexity: O(1) constant -- storing only some additional integers
pub mod first_attempt {
    pub struct Solution;
    impl Solution {
        pub fn jump(nums: Vec<i32>) -> i32 {
            let mut result: i32 = 0;
            let mut goal: usize = nums.len() - 1;

            while goal != 0 {
                for i in 0..goal {
                    if i.saturating_add(nums[i] as usize) >= goal {
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
