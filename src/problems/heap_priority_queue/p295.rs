#[allow(dead_code)]
pub mod two_binary_heaps {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    #[derive(Default)]
    struct MedianFinder {
        pub small: BinaryHeap<i32>,
        pub large: BinaryHeap<Reverse<i32>>,
    }

    impl MedianFinder {
        fn new() -> Self {
            Self::default()
        }

        fn add_num(&mut self, num: i32) {
            self.small.push(num);

            if self.small.len() > self.large.len() + 1
                && let Some(smol) = self.small.pop()
            {
                self.large.push(Reverse(smol));
            }

            if self.small.peek() > self.large.peek().map(|x| x.0).as_ref()
                && let Some(n) = self.small.pop()
            {
                self.large.push(Reverse(n));
            }

            if self.large.len() > self.small.len() + 1
                && let Some(n) = self.large.pop()
            {
                self.small.push(n.0);
            }
        }

        fn is_even(&self) -> bool {
            (self.small.len() + self.large.len()) % 2 == 0
        }

        fn find_median(&self) -> f64 {
            if self.is_even() {
                return (*self.small.peek().clone().unwrap() as f64
                    + self.large.peek().clone().unwrap().0 as f64)
                    / 2f64;
            }

            if self.small.len() > self.large.len() {
                return *self.small.peek().unwrap() as f64;
            }

            self.large.peek().unwrap().0 as f64
        }
    }
}
