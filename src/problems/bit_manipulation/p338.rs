pub mod dynamic_programming {
    pub struct Solution;
    impl Solution {
        pub fn count_bits(n: i32) -> Vec<i32> {
            let mut result: Vec<i32> = Vec::with_capacity((n + 1) as usize);

            for i in 0..=n {
                if i == 0 {
                    result.push(0i32);
                    continue;
                }

                result.push(result[(i >> 1) as usize] + (i & 1));
            }

            result
        }
    }
}

pub mod iterative {
    pub struct Solution {}
    impl Solution {
        pub fn count_bits(n: i32) -> Vec<i32> {
            let mut result: Vec<i32> = Vec::with_capacity((n + 1) as usize);

            for i in 0..n + 1 {
                let mut count: i32 = 0;
                let mut x = i;
                while x != 0 {
                    count += x & 1;
                    x >>= 1;
                }
                result.push(count);
            }

            result
        }
    }
}
