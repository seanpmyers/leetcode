pub mod tree {
    pub struct Solution;
    pub struct Calendar {
        pub time: (i32, i32),
        pub left: Option<Box<Calendar>>,
        pub right: Option<Box<Calendar>>,
    }

    impl Calendar {
        pub fn new(start: i32, end: i32) -> Self {
            Self {
                time: (start, end),
                left: None,
                right: None,
            }
        }

        pub fn update(&mut self, start: i32, end: i32) -> bool {
            if !(start >= self.time.1 || end <= self.time.0) {
                return false;
            }
            if end <= self.time.0 {
                let Some(left) = self.left.as_mut() else {
                    self.left = Some(Box::new(Calendar::new(start, end)));
                    return true;
                };
                return left.update(start, end);
            }

            if start >= self.time.1 {
                let Some(right) = self.right.as_mut() else {
                    self.right = Some(Box::new(Calendar::new(start, end)));
                    return true;
                };
                return right.update(start, end);
            }

            true
        }
    }

    impl Solution {
        pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
            if intervals.is_empty() {
                return true;
            }
            let mut calendar: Calendar = Calendar::new(intervals[0][0], intervals[0][1]);

            for interval in intervals.iter().skip(1) {
                let start: i32 = interval[0];
                let end: i32 = interval[1];

                if !calendar.update(start, end) {
                    return false;
                }
            }

            true
        }
    }
}
pub mod sorting {
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
}
