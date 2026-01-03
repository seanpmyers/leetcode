pub mod n_log_n {
    pub struct Solution;
    use std::collections::BinaryHeap;
    impl Solution {
        pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
            let mut heap: BinaryHeap<(std::cmp::Reverse<i32>, (i32, i32))> =
                BinaryHeap::with_capacity(points.len());

            for point in points.iter() {
                let x = point[0];
                let y = point[1];

                let distance = Self::distance((0i32, 0i32), (x, y));
                heap.push((std::cmp::Reverse(distance as i32), (x, y)));
            }

            let mut result: Vec<Vec<i32>> = Vec::with_capacity(k as usize);

            for _ in 0..k {
                let top = heap.pop().unwrap();
                result.push(vec![top.1.0, top.1.1]);
            }

            result
        }

        pub fn distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
            (x1.saturating_sub(x2)).pow(2u32) + (y1.saturating_sub(y2).pow(2u32))
        }
    }
}
