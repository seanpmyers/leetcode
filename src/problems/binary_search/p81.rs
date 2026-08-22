pub mod first {
    pub struct Solution;
    use std::cmp::Ordering;
    impl Solution {
        pub fn search(nums: Vec<i32>, target: i32) -> bool {
            let mut l: usize = 0;
            let mut r: usize = nums.len() - 1;

            while l <= r {
                let middle: usize = l.midpoint(r);
                if nums[middle] == target {
                    return true;
                }
                match nums[l].cmp(&nums[middle]) {
                    Ordering::Less => {
                        if nums[l] <= target && target <= nums[middle] {
                            r = middle - 1;
                            continue;
                        }
                        l = middle + 1;
                    }
                    Ordering::Greater => {
                        if nums[middle] <= target && target <= nums[r] {
                            l = middle + 1;
                            continue;
                        }
                        r = middle - 1;
                    }
                    Ordering::Equal => l += 1,
                };
            }

            false
        }
    }
}
