pub mod cycle_sort {
    pub struct Solution;
    impl Solution {
        pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
            let n: usize = nums.len();

            let mut i: usize = 0;

            while i < n {
                if nums[i] <= 0 || nums[i] as usize > n {
                    i += 1;
                    continue;
                }

                let index: usize = nums[i] as usize - 1;
                if nums[index] == nums[i] {
                    i += 1;
                    continue;
                }

                nums.swap(index, i);
            }

            for x in 0..n {
                if nums[x] != (x + 1) as i32 {
                    return (x + 1) as i32;
                }
            }

            (n + 1) as i32
        }
    }
}
