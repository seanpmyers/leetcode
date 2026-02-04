pub mod dynamic {
    pub struct Solution {}
    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let mut result: i32 = 0;
            let mut current = prices[0];
            for v in prices.into_iter() {
                current = current.min(v);
                result = result.max(v - current);
            }

            result
        }
    }
}
pub mod sliding_window {
    pub struct Solution {}
    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let mut result: i32 = 0i32;

            let mut l: usize = 0;
            let mut r: usize = 1;

            while r < prices.len() {
                let left = prices[l];
                let right = prices[r];
                match left < right {
                    true => result = result.max(right - left),
                    false => l = r,
                };
                r += 1;
            }

            result
        }
    }
}
