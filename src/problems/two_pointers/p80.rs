pub mod linear_simplest {
    pub struct Solution;
    impl Solution {
        pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            if nums.len() <= 2 {
                return nums.len() as i32;
            }
            let mut k: usize = 2usize;
            for i in 2..nums.len() {
                if nums[i] != nums[k - 2] {
                    nums[k] = nums[i];
                    k += 1;
                }
            }

            k as i32
        }
    }
}
pub mod linear {
    pub struct Solution;
    impl Solution {
        pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            if nums.len() <= 2 {
                return nums.len() as i32;
            }
            let mut k: usize = 1usize;
            let mut count: usize = 1;
            let mut previous = nums[0];
            for i in 1..nums.len() {
                if previous == nums[i] {
                    count += 1;
                } else {
                    count = 1;
                }

                previous = nums[i];

                nums[k] = nums[i];

                if count <= 2 {
                    k += 1;
                }
            }

            k as i32
        }
    }
}

pub mod quadratic {
    pub struct Solution;
    impl Solution {
        pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            let mut l: usize = 0;
            let mut r: usize = 0;
            let mut count: usize = 0;
            let mut k: usize = nums.len();
            while l < k {
                while r < k && nums[r] == nums[l] {
                    count += 1;
                    r += 1;
                }

                if count <= 2 {
                    l = r;
                    count = 0;
                    continue;
                }

                count -= 2;
                k -= count;
                l += 2;
                for _ in 0..count {
                    let mut current = l;
                    for i in r..nums.len() {
                        nums.swap(current, i);
                        current = i;
                    }
                    l += 1;
                }
                r = r - count;
                l = r;
                count = 0;
            }

            k as i32
        }
    }
}
