pub mod count {
    pub struct Solution;
    impl Solution {
        pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
            if intervals.len() == 1 {
                return 0i32;
            }
            intervals.sort();

            let mut result: i32 = 0;
            let mut x: &Vec<i32> = &intervals[intervals.len() - 1];
            for y in intervals.iter().rev().skip(1) {
                if x[0] >= y[1] {
                    x = y;
                    continue;
                }
                result += 1;
            }

            result
        }
    }
}
pub mod pop {
    pub struct Solution;
    impl Solution {
        pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
            if intervals.len() == 1 {
                return 0i32;
            }
            let len: usize = intervals.len();
            let mut result: Vec<Vec<i32>> = vec![];
            intervals.sort();
            let mut current: Vec<i32> = intervals.pop().unwrap();

            while let Some(interval) = intervals.pop() {
                if current[0] >= interval[1] {
                    result.push(current);
                    current = interval;
                    continue;
                }
            }
            result.push(current);

            (len - result.len()) as i32
        }
    }
}
pub mod first {
    pub struct Solution;
    impl Solution {
        pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
            if intervals.len() == 1 {
                return 0i32;
            }

            intervals.sort();
            let mut end: i32 = intervals[0][1];
            let mut result: i32 = 0;

            for i in 1..intervals.len() {
                if end > intervals[i][0] {
                    end = intervals[i][1].min(end);
                    result += 1;
                    continue;
                }
                end = intervals[i][1];
            }

            result
        }
    }
}
