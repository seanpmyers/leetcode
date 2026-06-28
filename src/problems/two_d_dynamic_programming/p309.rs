pub mod optimal {
    pub struct Solution;
    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            if prices.len() == 1 {
                return 0i32;
            }

            let mut dp1_buy: i32 = 0i32;
            let mut dp1_sell: i32 = 0i32;
            let mut dp2_buy: i32 = 0i32;

            for &price in prices.iter().rev() {
                let dp_buy = (dp1_sell - price).max(dp1_buy);
                let dp_sell = (dp2_buy + price).max(dp1_sell);
                dp2_buy = dp1_buy;
                dp1_sell = dp_sell;
                dp1_buy = dp_buy;
            }

            dp1_buy
        }
    }
}
