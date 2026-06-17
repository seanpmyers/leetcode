pub mod dynamic_progrmming {
    pub struct Solution;

    impl Solution {
        pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
            let mut travel_days: [bool; 366] = [false; 366usize];
            for &day in &days {
                travel_days[day as usize] = true;
            }

            let mut dp: [i32; 366usize] = [0i32; 366usize];

            for i in 1..366 {
                if !travel_days[i] {
                    dp[i] = dp[i - 1];
                    continue;
                }

                let one = dp[i.saturating_sub(1usize)] + costs[0];
                let seven = dp[i.saturating_sub(7usize)] + costs[1];
                let thirty = dp[i.saturating_sub(30usize)] + costs[2];

                dp[i] = one.min(seven).min(thirty);
            }

            dp[365]
        }
    }
}
