pub struct Solution {}
impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut reversed: u32 = 0;
        for _ in 0..32 {
            reversed <<= 1;
            reversed += 1 & x;
            x >>= 1;
        }
        reversed
    }
}
