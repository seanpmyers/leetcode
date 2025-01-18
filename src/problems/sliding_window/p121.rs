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
