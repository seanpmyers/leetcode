pub struct Solution;
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left: Vec<i32> = vec![0; nums.len()];
        let mut right: Vec<i32> = vec![0; nums.len()];
        let len = nums.len();

        for i in 1..len {
            left[i] = left[i - 1] + nums[i - 1];
            right[len - 1 - i] = right[len - i] + nums[len - i]
        }
        for i in 0..len {
            if i == 0 {
                if right[i] == 0 {
                    return i as i32;
                }
                continue;
            }
            if i == len - 1 {
                if left[i] == 0 {
                    return i as i32;
                }
                continue;
            }
            if left[i] == right[i] {
                return i as i32;
            }
        }

        -1i32
    }
}
