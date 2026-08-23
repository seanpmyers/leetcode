pub mod third {
    pub struct Solution;
    impl Solution {
        pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
            intervals.sort();
            while let Some(mut next) = intervals.pop() {
                if result.is_empty() || result.last().is_some_and(|x| x[0] > next[1]) {
                    result.push(next);
                    continue;
                }

                let last = result.pop().unwrap();
                next[0] = last[0].min(next[0]);
                next[1] = last[1].max(next[1]);
                intervals.push(next);
            }

            result
        }
    }
}
pub mod second {
    pub struct Solution;
    impl Solution {
        pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
            intervals.sort();
            let mut current = intervals[0].clone();
            for interval in intervals.iter().skip(1) {
                if current[0] < interval[0] && current[1] < interval[0] {
                    result.push(current.clone());
                    current = interval.clone();
                    continue;
                }
                current[1] = current[1].max(interval[1]);
            }

            result.push(current);

            result
        }
    }
}
pub mod first {
    pub struct Solution;
    impl Solution {
        pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            if intervals.len() == 1 {
                return intervals;
            }

            intervals.sort();
            let mut result: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
            let (mut start, mut end): (i32, i32) = (intervals[0][0], intervals[0][1]);
            for i in 1..intervals.len() {
                let current: &Vec<i32> = &intervals[i];

                if current[0] <= end {
                    end = current[1].max(end);
                    continue;
                }

                result.push(vec![start, end]);
                start = current[0];
                end = current[1];
            }

            result.push(vec![start, end]);

            result
        }
    }
}
