pub struct Solution;
use std::cmp::Ordering;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let x = x as i64;
        let mut left: i64 = 1;
        let mut right: i64 = x;

        while left < right {
            let middle = left.midpoint(right);
            match (middle * middle).cmp(&x) {
                Ordering::Greater => right = middle - 1,
                Ordering::Less => left = middle,
                Ordering::Equal => return middle as i32,
            };
            if left == right - 1 {
                break;
            }
        }

        if right * right <= x {
            return right as i32;
        }

        left as i32
    }
}
