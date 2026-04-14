pub mod linear_hash_set {
    pub struct Solution;
    use std::collections::HashSet;
    impl Solution {
        pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
            if nums.len() <= 1 {
                return false;
            }

            let k: usize = k as usize;
            let mut set: HashSet<i32> = HashSet::with_capacity(k + 1);

            let mut l: usize = 0;
            let mut r: usize = 1;
            set.insert(nums[l]);

            while r < nums.len() && l.abs_diff(r) < k {
                if !set.insert(nums[r]) {
                    return true;
                }
                r += 1;
            }

            while l < nums.len() && r < nums.len() {
                if l != r && !set.insert(nums[r]) && l.abs_diff(r) <= k {
                    return true;
                }

                set.remove(&nums[l]);
                l += 1;
                r += 1;
            }

            false
        }
    }
}

pub mod sliding_window_brute_force {
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
