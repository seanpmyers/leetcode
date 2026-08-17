pub mod reverse {
    pub struct Solution;
    impl Solution {
        pub fn rotate(nums: &mut Vec<i32>, k: i32) {
            let n: usize = nums.len();
            if n <= 1 {
                return;
            }
            let k: usize = k as usize % n;

            nums.reverse();
            nums[..k].reverse();
            nums[k..].reverse();
        }
    }
}
pub mod constant_space {
    pub struct Solution;
    impl Solution {
        pub fn rotate(nums: &mut Vec<i32>, k: i32) {
            let n: usize = nums.len();
            if n <= 1 {
                return;
            }
            let k: usize = k as usize;

            let mut start: usize = 0;
            let mut count: usize = 0;

            while count < n {
                let mut l = start;
                let mut r: usize = (l + k) % n;
                let mut temp: i32 = nums[l];
                std::mem::swap(&mut nums[r], &mut temp);
                l = r;
                r = (l + k) % n;
                count += 1;
                while l != start {
                    std::mem::swap(&mut nums[r], &mut temp);
                    l = r;
                    r = (l + k) % n;
                    count += 1;
                }
                start += 1;
            }
        }
    }
}
pub mod temp_array {
    pub struct Solution;
    impl Solution {
        pub fn rotate(nums: &mut Vec<i32>, k: i32) {
            let n: usize = nums.len();
            let k: usize = k as usize;

            let mut temp: Vec<i32> = vec![0; n];

            for i in 0..n {
                let xi = (i + k) % n;
                let x: i32 = nums[i];
                temp[xi] = x;
            }

            *nums = temp;
        }
    }
}
