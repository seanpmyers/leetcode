pub struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start: usize = 0;
        let mut end: usize = nums.len();

        while start < end {
            let middle = middle(start, end);
            match nums[middle].cmp(&target) {
                std::cmp::Ordering::Equal => return middle as i32,
                std::cmp::Ordering::Greater => end = middle,
                std::cmp::Ordering::Less => start = middle + 1,
            }
        }
        -1i32
    }
}

pub fn middle(start: usize, end: usize) -> usize {
    start + (end - start) / 2
}
