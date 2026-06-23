pub mod iterative {
    pub struct Solution;
    use std::cmp::Ordering;
    impl Solution {
        pub fn is_monotonic(nums: Vec<i32>) -> bool {
            if nums.len() == 1 {
                return true;
            }
            let mut previous = nums[1];
            let mut direction = nums[0].cmp(&nums[1]);
            for i in 2..nums.len() {
                let current = previous.cmp(&nums[i]);
                previous = nums[i];
                if direction == Ordering::Equal {
                    direction = current;
                    continue;
                }
                if current != direction && current != Ordering::Equal {
                    return false;
                }
            }

            true
        }
    }
}
