pub mod bucket_sort {
    pub struct Solution;
    impl Solution {
        pub fn sort_colors(nums: &mut Vec<i32>) {
            let mut count: [usize; 3usize] = [0usize; 3usize];
            for i in 0..nums.len() {
                count[nums[i] as usize] += 1;
            }

            let mut i: usize = 0;
            for x in 0..count.len() {
                let n: usize = count[x];
                for _ in 0..n {
                    nums[i] = x as i32;
                    i += 1;
                }
            }
        }
    }
}
