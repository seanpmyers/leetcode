pub mod append {
    pub struct Solution;
    impl Solution {
        pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
            let mut x = nums.clone();
            x.append(&mut nums);
            x
        }
    }
}

pub mod iterative {
    pub struct Solution;
    impl Solution {
        pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
            let n: usize = nums.len();

            for i in 0..n {
                nums.push(nums[i]);
            }

            nums
        }
    }
}
