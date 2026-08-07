pub mod second {
    pub struct Solution;
    impl Solution {
        pub fn max_area(height: Vec<i32>) -> i32 {
            let mut result: i32 = 0;

            if height.len() < 2 {
                return result;
            }

            let mut l: usize = 0;
            let mut r: usize = height.len() - 1;

            while l < r {
                let h: i32 = height[l].min(height[r]);
                let w: i32 = (r - l) as i32;
                result = result.max(h * w);
                if height[l] >= height[r] {
                    r -= 1;
                    continue;
                }
                l += 1;
            }

            result
        }
    }
}
pub mod first {
    pub struct Solution;
    use std::cmp::Ordering;
    impl Solution {
        pub fn max_area(height: Vec<i32>) -> i32 {
            let mut l: usize = 0;
            let mut r: usize = height.len() - 1;
            let mut result: i32 = 0;

            while l < r {
                result = result.max(height[l].min(height[r]) * (l.abs_diff(r) as i32));

                match height[l].cmp(&height[r]) {
                    Ordering::Equal | Ordering::Less => l += 1,
                    Ordering::Greater => r -= 1,
                };
            }

            result
        }
    }
}
