pub mod iterative {
    pub struct Solution;
    impl Solution {
        pub fn climb_stairs(n: i32) -> i32 {
            if n == 1 {
                return 1;
            }

            if n == 2 {
                return 2;
            }

            let (mut x, mut y) = (1, 2);
            for _ in 2..n {
                let temp = x + y;
                x = y;
                y = temp;
            }

            y
        }
    }
}
