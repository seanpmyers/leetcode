// #arrays #hashing
pub struct Solution {}
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![1i32; nums.len()];
        let len: usize = nums.len() - 1;
        let mut left: i32 = 1;
        let mut right: i32 = 1;

        for i in 0..len {
            result[i] *= left;
            result[len - i] *= right;
            left *= nums[i];
            right *= nums[len - i];
        }

        result[0] *= right;
        result[len] *= left;

        result
    }
}
