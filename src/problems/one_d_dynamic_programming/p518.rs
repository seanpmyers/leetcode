pub mod dynamic_programming {
    pub struct Solution;
    impl Solution {
        pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
            let amount: usize = amount as usize;

            let mut dp: Vec<usize> = vec![0usize; amount + 1];
            dp[0] = 1;

            let mut i: usize = 0;

            for &coin in &coins {
                let coin = coin as usize;
                for i in coin..=amount {
                    dp[i] += dp[i - coin];
                }
            }

            dp[amount] as i32
        }
    }
}
