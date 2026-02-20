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
