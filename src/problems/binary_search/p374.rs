pub struct Solution;
pub fn guess(_n: i32) -> i32 {
    return 0;
}
impl Solution {
    pub fn guess_number(n: i32) -> i32 {
        if guess(1i32) == 0 {
            return 1i32;
        }
        if guess(n) == 0 {
            return n;
        }
        let mut left: i32 = 1;
        let mut right: i32 = n;

        while left < right {
            let middle: i32 = Self::middle(left, right);
            match guess(middle) {
                -1 => right = middle,
                1 => left = middle + 1,
                _ => return middle,
            }
        }

        -1i32
    }

    pub fn middle(start: i32, end: i32) -> i32 {
        let start: u32 = start as u32;
        let end: u32 = end as u32;

        ((start + end) / 2) as i32
    }
}
