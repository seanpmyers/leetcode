pub mod binary_search {
    pub struct Solution;
    impl Solution {
        pub fn find_duplicate(nums: Vec<i32>) -> i32 {
            let mut l: i32 = 1 as i32;
            let mut r: i32 = (nums.len() - 1) as i32;

            while l < r {
                let middle: i32 = l.midpoint(r);
                let count: i32 = nums.iter().filter(|n| **n <= middle as i32).count() as i32;
                if count > middle {
                    r = middle;
                    continue;
                }
                l = middle + 1;
            }

            l
        }
    }
}
pub mod fast_and_slow {
    pub struct Solution;
    impl Solution {
        pub fn find_duplicate(nums: Vec<i32>) -> i32 {
            let mut slow: usize = nums[0usize] as usize;
            let mut fast: usize = nums[nums[0usize] as usize] as usize;

            while slow != fast {
                slow = nums[slow] as usize;
                fast = nums[nums[fast] as usize] as usize;
            }

            let mut slow2: usize = 0usize;

            while slow != slow2 {
                slow = nums[slow] as usize;
                slow2 = nums[slow2] as usize;
            }
            return slow as i32;
        }
    }
}
