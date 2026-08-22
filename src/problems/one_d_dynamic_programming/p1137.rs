pub mod first {
    pub struct Solution;
    impl Solution {
        pub fn tribonacci(n: i32) -> i32 {
            if n == 0 {
                return 0i32;
            }
            if n <= 2 {
                return 1;
            }
            let mut x: i32 = 0;
            let mut y: i32 = 1;
            let mut z: i32 = 1;

            for i in 3..=n {
                let w = x + y + z;
                x = y;
                y = z;
                z = w;
            }

            z
        }
    }
}
