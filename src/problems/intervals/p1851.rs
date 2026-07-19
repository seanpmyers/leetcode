pub mod slow {
    pub struct Solution;
    impl Solution {
        pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
            let mut result: Vec<i32> = vec![-1i32; queries.len()];

            let mut intervals: Vec<(i32, i32, i32)> = intervals
                .into_iter()
                .map(|x| (x[1] - x[0] + 1i32, x[0], x[1]))
                .collect();

            intervals.sort();

            for (i, query) in queries.iter().enumerate() {
                for (size, start, end) in intervals.iter() {
                    if !(start <= query && query <= end) {
                        continue;
                    }
                    result[i] = *size;
                    break;
                }
            }

            result
        }
    }
}
