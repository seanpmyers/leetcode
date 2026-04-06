pub mod binary_indexing {
    pub struct Solution;
    impl Solution {
        pub fn reverse_bits(n: i32) -> i32 {
            let mut result: i32 = 0;

            for i in 0..32 {
                let bit: i32 = (n >> i) & 1;
                result += bit << (31 - i)
            }

            result
        }
    }
}

pub mod mutate_n {
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
}
