pub struct NumArray {
    pub nums: Vec<i32>,
    pub sum: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut sum: Vec<i32> = vec![0; nums.len()];
        for i in 0..nums.len() {
            if i == 0 {
                sum[i] = nums[i];
                continue;
            }
            sum[i] = sum[i - 1] + nums[i];
        }

        Self { nums, sum }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        let mut result = self.sum[right];
        if left > 0 {
            result -= self.sum[left - 1];
        }
        result
    }
}
