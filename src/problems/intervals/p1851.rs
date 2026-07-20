pub mod heap {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    impl Solution {
        pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
            let mut result: Vec<i32> = vec![-1i32; queries.len()];

            let mut intervals: Vec<(i32, i32, i32)> = intervals
                .into_iter()
                .map(|x| (x[0], x[1], x[1] - x[0] + 1i32))
                .collect();
            let mut queries: Vec<(usize, i32)> = queries.into_iter().enumerate().collect();
            intervals.sort();
            queries.sort_by(|x, y| x.1.cmp(&y.1));

            let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

            let mut index: usize = 0;

            for (i, query) in queries.iter() {
                while index < intervals.len() && intervals[index].0 <= *query {
                    let x = &intervals[index];
                    heap.push(Reverse((x.2, x.1)));
                    index += 1;
                }

                while heap.peek().is_some_and(|x| x.0.1 < *query) {
                    heap.pop();
                }

                if let Some(top) = heap.peek() {
                    result[*i] = top.0.0;
                }
            }

            result
        }
    }
}
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
