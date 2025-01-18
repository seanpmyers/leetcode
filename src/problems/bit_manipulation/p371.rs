pub struct Solution {}
impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let carry = a & b;
            a ^= b;
            b = carry << 1;
        }

        a
    }
}
