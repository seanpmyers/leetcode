pub struct Solution;
impl Solution {
    pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() == 1 {
            return 1i32;
        }
        intervals.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
        let mut rooms: Vec<Vec<(i32, i32)>> = vec![vec![(intervals[0][0], intervals[0][1])]];

        for i in 1..intervals.len() {
            let current: &Vec<i32> = &intervals[i];
            let mut added: bool = false;
            for r in 0..rooms.len() {
                if rooms[r].last().is_some_and(|m| m.1 <= current[0]) {
                    rooms[r].push((current[0], current[1]));
                    added = true;
                    break;
                }
            }
            if !added {
                rooms.push(vec![(current[0], current[1])]);
            }
        }

        rooms.len() as i32
    }
}
