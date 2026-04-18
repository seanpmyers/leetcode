pub mod linear {
    pub struct Solution;
    impl Solution {
        pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            if nums.len() <= 1 {
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

                if count <= 1 {
                    k += 1;
                }
            }

            k as i32
        }
    }
}
