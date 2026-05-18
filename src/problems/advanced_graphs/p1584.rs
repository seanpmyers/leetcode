pub mod almost_prims {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashSet};
    impl Solution {
        pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
            let mut n: usize = points.len();

            let mut result: i32 = 0i32;
            let mut visited: HashSet<usize> = HashSet::new();
            let mut heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();

            heap.push((Reverse(0), 0));

            while let Some((Reverse(distance), index)) = heap.pop() {
                if !visited.insert(index) {
                    continue;
                }
                n -= 1;
                result += distance;
                if n == 0 {
                    return result;
                }

                for i in 0..points.len() {
                    if visited.contains(&i) {
                        continue;
                    }
                    heap.push((
                        Reverse(
                            points[index][0].abs_diff(points[i][0]) as i32
                                + points[index][1].abs_diff(points[i][1]) as i32,
                        ),
                        i,
                    ));
                }
            }

            result
        }
    }
}
