pub struct Solution;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 2 {
            match nums[0].cmp(&nums[1]) {
                std::cmp::Ordering::Less => return nums[0],
                std::cmp::Ordering::Equal => return -1i32,
                std::cmp::Ordering::Greater => return nums[1],
            }
        }
        pub fn middle(start: usize, end: usize) -> usize {
            start + (end - start) / 2
        }
        let mut s: usize = 0;
        let mut e: usize = nums.len() - 1;
        while s < e {
            let middle: usize = middle(s, e);
            if nums[middle] < nums[e] {
                e = middle;
            } else {
                s = middle + 1;
            }
        }
        nums[s]
    }
}
