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
