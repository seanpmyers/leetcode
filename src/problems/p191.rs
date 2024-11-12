pub struct Solution {}
impl Solution {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut result: i32 = 0i32;
        while n != 0 {
            result += n & 1;
            n >>= 1;
        }

        result
    }
}
