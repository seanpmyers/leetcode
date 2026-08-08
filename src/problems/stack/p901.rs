#[allow(dead_code)]
pub mod monotonic_decreasing_stack {
    struct StockSpanner {
        pub history: Vec<(i32, i32)>,
    }

    impl StockSpanner {
        fn new() -> Self {
            Self {
                history: Vec::new(),
            }
        }

        fn next(&mut self, price: i32) -> i32 {
            let mut result: i32 = 1;

            while self.history.last().is_some_and(|x| x.0 <= price) {
                let last: (i32, i32) = self.history.pop().unwrap();
                result += last.1;
            }
            self.history.push((price, result));

            result
        }
    }
}
