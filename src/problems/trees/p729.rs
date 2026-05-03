#[allow(dead_code)]
pub mod segment_tree {
    #[derive(Debug)]
    pub struct SegmentTree {
        pub left: Option<Segment>,
        pub right: Option<Segment>,
        pub start: i32,
        pub end: i32,
        pub booked: bool,
    }

    pub type Segment = Box<SegmentTree>;

    impl SegmentTree {
        pub fn new() -> Self {
            Self {
                left: None,
                right: None,
                start: 0i32,
                end: 1_000_000_000i32,
                booked: false,
            }
        }

        fn query(&self, start: i32, end: i32) -> bool {
            if start >= self.end || end <= self.start {
                return false;
            }

            if !self.booked {
                return false;
            }

            if self.left.is_none() && self.right.is_none() {
                return true;
            }

            let mut overlap = false;

            if let Some(ref left) = self.left {
                overlap |= left.query(start, end);
            }

            if !overlap && let Some(ref right) = self.right {
                overlap |= right.query(start, end);
            }

            overlap
        }

        fn insert(&mut self, start: i32, end: i32) {
            if start >= self.end || end <= self.start {
                return;
            }

            self.booked = true;

            if start <= self.start && end >= self.end {
                return;
            }

            let middle = self.end.midpoint(self.start);

            if self.left.is_none() {
                self.left = Some(Box::new(SegmentTree {
                    left: None,
                    right: None,
                    start: self.start,
                    end: middle,
                    booked: false,
                }));
            }
            if self.right.is_none() {
                self.right = Some(Box::new(SegmentTree {
                    left: None,
                    right: None,
                    start: middle,
                    end: self.end,
                    booked: false,
                }));
            }

            if let Some(ref mut left) = self.left {
                left.insert(start, end);
            }
            if let Some(ref mut right) = self.right {
                right.insert(start, end);
            }
        }

        pub fn update(&mut self, start: i32, end: i32) -> bool {
            if self.query(start, end) {
                return false;
            }
            self.insert(start, end);
            true
        }
    }

    struct MyCalendar {
        pub tree: SegmentTree,
    }

    impl MyCalendar {
        fn new() -> Self {
            Self {
                tree: SegmentTree::new(),
            }
        }

        fn book(&mut self, start: i32, end: i32) -> bool {
            self.tree.update(start, end)
        }
    }
}
#[allow(dead_code)]
pub mod imbalanced {
    pub struct SegmentTree {
        pub left: Option<Segment>,
        pub right: Option<Segment>,
        pub start: i32,
        pub end: i32,
    }

    type Segment = Box<SegmentTree>;

    impl SegmentTree {
        pub fn update(&mut self, start: i32, end: i32) -> bool {
            if start < self.end && end > self.start {
                return false;
            }

            if start >= self.end {
                if let Some(right) = self.right.as_mut() {
                    return right.update(start, end);
                }

                self.right = Some(Box::new(Self {
                    left: None,
                    right: None,
                    start,
                    end,
                }));

                return true;
            }

            if end <= self.start {
                if let Some(left) = self.left.as_mut() {
                    return left.update(start, end);
                }

                self.left = Some(Box::new(Self {
                    left: None,
                    right: None,
                    start,
                    end,
                }));

                return true;
            }

            false
        }
    }

    struct MyCalendar {
        pub tree: Option<Segment>,
    }

    impl MyCalendar {
        fn new() -> Self {
            Self { tree: None }
        }

        fn book(&mut self, start_time: i32, end_time: i32) -> bool {
            let Some(tree) = self.tree.as_mut() else {
                self.tree = Some(Box::new(SegmentTree {
                    left: None,
                    right: None,
                    start: start_time,
                    end: end_time,
                }));
                return true;
            };

            tree.update(start_time, end_time)
        }
    }
}
