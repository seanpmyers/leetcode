pub struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            if nums[0] == target {
                return 0i32;
            } else {
                return -1i32;
            }
        }

        let mut s: usize = 0;
        let mut e: usize = nums.len() - 1;

        while s < e {
            let middle: usize = s + (e - s) / 2;
            if nums[middle] == target {
                return middle as i32;
            }
            if nums[s] == target {
                return s as i32;
            }
            if nums[e] == target {
                return e as i32;
            }

            if nums[s] <= nums[middle] {
                if target > nums[middle] || target < nums[s] {
                    s = middle + 1;
                } else {
                    e = middle - 1;
                }
            } else {
                if target < nums[middle] || target > nums[e] {
                    e = middle - 1;
                } else {
                    s = middle + 1;
                }
            }
        }
        if nums[s] == target {
            return s as i32;
        }
        -1i32
    }
}
