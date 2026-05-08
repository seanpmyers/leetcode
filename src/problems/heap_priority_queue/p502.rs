pub mod two_heaps {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    impl Solution {
        pub fn find_maximized_capital(
            mut k: i32,
            mut w: i32,
            profits: Vec<i32>,
            capital: Vec<i32>,
        ) -> i32 {
            let n: usize = profits.len();
            let mut p_heap: BinaryHeap<(i32, usize)> = BinaryHeap::with_capacity(n);
            let mut c_heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::with_capacity(n);

            for i in 0..n {
                c_heap.push((Reverse(capital[i]), i));
            }

            while k > 0 {
                while c_heap.peek().map(|(n, _)| n.0) <= Some(w)
                    && let Some((_, index)) = c_heap.pop()
                {
                    p_heap.push((profits[index], index));
                }

                if let Some(top) = p_heap.pop() {
                    w += top.0;
                }

                k -= 1;
            }

            w
        }
    }
}
