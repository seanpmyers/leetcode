pub mod second {
    pub struct Solution;
    use std::collections::HashSet;
    impl Solution {
        pub fn is_happy(mut n: i32) -> bool {
            let mut set: HashSet<i32> = HashSet::with_capacity(100);
            let mut next: i64;

            while set.insert(n) && n != 1 {
                next = 0;
                while n != 0 {
                    next += (n % 10).pow(2u32) as i64;
                    n /= 10;
                }
                n = next as i32;
            }

            n == 1
        }
    }
}
pub mod first {
    pub struct Solution {}
    impl Solution {
        pub fn is_happy(mut n: i32) -> bool {
            use std::collections::HashSet;
            let mut set: HashSet<i32> = HashSet::new();
            while n != 1 {
                let current = digit_square_sum(n);
                if !set.insert(current) {
                    return false;
                }
                n = current;
            }

            true
        }
    }

    pub fn digit_square_sum(mut n: i32) -> i32 {
        let mut result: i32 = 0;

        while n != 0 {
            result += (n % 10).pow(2);
            n = n / 10;
        }

        result
    }
}
