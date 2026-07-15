pub mod second {
    pub struct Solution;
    use std::collections::VecDeque;
    impl Solution {
        pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
            if intervals.is_empty() {
                return vec![new_interval];
            }
            let mut result: VecDeque<Vec<i32>> = VecDeque::with_capacity(intervals.len());

            while let Some(interval) = intervals.pop() {
                if new_interval[0] > interval[1] {
                    result.push_front(new_interval);
                    result.push_front(interval);
                    while let Some(i) = intervals.pop() {
                        result.push_front(i);
                    }
                    return Vec::from(result);
                }
                if new_interval[0] >= interval[0] && new_interval[1] <= interval[1] {
                    result.push_front(interval);
                    while let Some(i) = intervals.pop() {
                        result.push_front(i);
                    }
                    return Vec::from(result);
                }
                if new_interval[1] < interval[0] && new_interval[0] < interval[1] {
                    result.push_front(interval);
                    continue;
                }

                new_interval[1] = new_interval[1].max(interval[1]);
                new_interval[0] = new_interval[0].min(interval[0]);
                while intervals.last().is_some_and(|x| x[1] >= new_interval[0])
                    && let Some(end) = intervals.pop()
                {
                    if end[0] < new_interval[0] {
                        new_interval[0] = new_interval[0].min(end[0]);
                        break;
                    }
                }
                result.push_front(new_interval);

                while let Some(i) = intervals.pop() {
                    result.push_front(i);
                }
                return Vec::from(result);
            }

            result.push_front(new_interval);

            Vec::from(result)
        }
    }
}
pub mod clean {
    pub struct Solution;
    impl Solution {
        pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
            if intervals.is_empty() {
                return vec![new_interval];
            }
            let mut result: Vec<Vec<i32>> = Vec::with_capacity(intervals.len() + 1);
            let mut added: bool = false;
            for current in intervals.into_iter() {
                if added {
                    result.push(current);
                    continue;
                }

                if new_interval[1] < current[0] {
                    added = true;
                    result.push(new_interval.clone());
                    result.push(current);
                    continue;
                }

                let overlapping: bool = new_interval[0] == current[1]
                    || (new_interval[0] <= current[1] && new_interval[1] >= current[0])
                    || (new_interval[1] >= current[0] && new_interval[0] <= current[1]);

                if overlapping {
                    new_interval[0] = new_interval[0].min(current[0]);
                    new_interval[1] = new_interval[1].max(current[1]);
                    continue;
                }

                result.push(current);
            }

            if !added {
                result.push(new_interval.clone());
            }

            result
        }
    }
}
