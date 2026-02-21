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
