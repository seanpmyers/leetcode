pub struct Solution;
impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        if intervals.len() < 2 {
            return true;
        }

        intervals.sort_by(|x, y| x[0].cmp(&y[0]));

        for i in 1..intervals.len() {
            let previous = &intervals[i - 1];
            let current = &intervals[i];
            if current[0] < previous[1] {
                return false;
            }
        }

        true
    }
}
