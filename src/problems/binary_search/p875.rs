pub mod second {
    pub struct Solution;
    use std::cmp::Ordering;
    impl Solution {
        pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
            let max: i32 = *piles.iter().max().unwrap_or(&i32::MAX);
            let mut result: i32 = max;
            let mut l: i32 = 0;
            let mut r: i32 = max;

            while l <= r {
                let k: f64 = l.midpoint(r) as f64;
                let mut hours: i32 = 0;

                for pile in piles.iter().map(|p| *p as f64) {
                    hours = hours.saturating_add((pile / k).ceil() as i32);
                }
                let k = k as i32;

                match hours.cmp(&h) {
                    Ordering::Equal | Ordering::Less => {
                        result = result.min(k);
                        r = k - 1;
                    }
                    Ordering::Greater => l = k + 1,
                };
            }

            result
        }
    }
}
pub mod first {
    pub struct Solution {}

    impl Solution {
        pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
            let calc = |k| {
                piles.iter().fold(0, |accumulator, x| {
                    accumulator + (x / k) + if x % k > 0 { 1 } else { 0 }
                })
            };
            let mut start: i32 = 1i32;
            let mut end: i32 = *piles.iter().max().unwrap_or(&i32::MAX);

            while start < end {
                let middle = middle(start, end);
                if calc(middle) <= h {
                    end = middle;
                } else {
                    start = middle + 1;
                }
            }
            start
        }
    }

    pub fn middle(start: i32, end: i32) -> i32 {
        start + (end - start) / 2i32
    }
}
