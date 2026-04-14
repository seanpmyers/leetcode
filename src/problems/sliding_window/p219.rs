pub mod sliding_window {
    pub struct Solution;
    impl Solution {
        pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
            if nums.len() <= 1 {
                return false;
            }

            let k: usize = k as usize;

            let mut l: usize = 0usize;
            let mut r: usize = 1usize;

            while l < nums.len() - 1 {
                if nums[l] == nums[r] && l.abs_diff(r) <= k && l != r {
                    return true;
                }

                if r < nums.len() - 1 && l.abs_diff(r + 1) <= k {
                    r += 1;
                    continue;
                }

                l = l + 1;

                if l + 1 < nums.len() - 1 {
                    r = l + 1;
                }
            }

            false
        }
    }
}
