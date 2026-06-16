pub mod queue {
    pub struct Solution;
    use std::collections::VecDeque;
    impl Solution {
        pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
            if amount == 0 {
                return 0;
            }

            let amount: usize = amount as usize;
            let mut queue: VecDeque<usize> = VecDeque::new();
            let mut seen: Vec<bool> = vec![false; amount + 1];
            seen[0] = true;
            queue.push_back(0usize);
            let mut result: i32 = 0i32;

            while !queue.is_empty() {
                let q_len: usize = queue.len();
                result += 1;
                for _ in 0..q_len {
                    let Some(previous) = queue.pop_front() else {
                        continue;
                    };
                    for &coin in &coins {
                        let mut next: usize = previous + coin as usize;
                        if next == amount {
                            return result;
                        }
                        if next > amount {
                            continue;
                        }
                        if seen[next] {
                            continue;
                        }
                        queue.push_back(next);
                        seen[next] = true;
                    }
                }
            }

            -1i32
        }
    }
}
pub mod recursion {
    pub struct Solution;
    impl Solution {
        pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
            if amount == 0i32 {
                return 0i32;
            }

            let mut dp: Vec<i32> =
                vec![amount.saturating_add(1); amount.saturating_add(1) as usize];

            dp[0] = 0;
            for i in 1..=amount {
                for j in 0..coins.len() {
                    if coins[j] <= i {
                        dp[i as usize] =
                            dp[i as usize].min(dp[(i.saturating_sub(coins[j])) as usize] + 1);
                    }
                }
            }

            match dp[amount as usize] > amount {
                true => -1i32,
                false => dp[amount as usize],
            }
        }
    }
}

pub mod first_attempt {
    pub struct Solution;
    impl Solution {
        pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
            if amount == 0i32 {
                return 0i32;
            }

            coins.sort_unstable_by(|a, b| b.cmp(a));

            let mut result: Option<i32> = None;
            for i in 0..coins.len() {
                let value = Self::backtrack(&coins[i..coins.len()], 0i32, amount);
                if value == -1 {
                    continue;
                }

                let Some(r) = result else {
                    result = Some(value);
                    continue;
                };

                result = Some(r.min(value));
            }

            result.unwrap_or(-1i32)
        }

        pub fn backtrack(coins: &[i32], result: i32, amount: i32) -> i32 {
            if coins.is_empty() {
                return -1i32;
            }
            let current: i32 = coins[0];
            let coins: &[i32] = &coins[1..coins.len()];
            if current > amount {
                Self::backtrack(coins, result, amount);
            }
            let division = amount / current;

            if amount % current == 0 {
                return result.saturating_add(amount / current);
            }

            let mut r: Option<i32> = None;
            for times in (0..=division).rev() {
                let remaining: i32 = amount.saturating_sub(current * times);
                let value: i32 = Self::backtrack(coins, result.saturating_add(times), remaining);
                if value != -1i32 {
                    match r {
                        Some(v) => r = Some(v.min(value)),
                        None => r = Some(value),
                    }
                }
            }

            r.unwrap_or(-1i32)
        }
    }
}
