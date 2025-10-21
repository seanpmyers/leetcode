pub struct Solution;
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow: usize = nums[0usize] as usize;
        let mut fast: usize = nums[nums[0usize] as usize] as usize;

        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
        }

        let mut slow2: usize = 0usize;

        while slow != slow2 {
            slow = nums[slow] as usize;
            slow2 = nums[slow2] as usize;
        }
        return slow as i32;
    }
}
