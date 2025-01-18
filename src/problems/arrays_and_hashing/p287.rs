pub struct Solution;
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        if nums.len() == 2 {
            return nums[0];
        }
        let mut slow: usize = nums[0] as usize;
        let mut fast: usize = nums[nums[0] as usize] as usize;
        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if slow == fast {
                break;
            }
        }
        let mut slow2: usize = 0;
        loop {
            slow = nums[slow] as usize;
            slow2 = nums[slow2] as usize;
            if slow == slow2 {
                return slow as i32;
            }
        }
    }
}
