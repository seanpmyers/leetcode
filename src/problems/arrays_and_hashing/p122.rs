pub mod first_attempt {
    pub struct Solution;
    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let mut result: i32 = 0;
            let mut previous: i32 = prices[0];
            for i in 1..prices.len() {
                let diff = prices[i] - previous;
                if diff > 0 {
                    result += diff;
                }
                previous = prices[i];
            }
            result
        }
    }
}
