pub mod prims_optimal {
    pub struct Solution;
    impl Solution {
        pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
            let n: usize = points.len();
            let mut result: i32 = 0;
            let mut current: usize = 0;
            let mut visited: Vec<bool> = vec![false; n];
            let mut min_distance: Vec<i32> = vec![10_000_000; n + 1];
            let mut count: usize = 0;

            while count < n - 1 {
                visited[current] = true;
                if n == 0 {
                    break;
                }
                let mut next: usize = points.len();
                for i in 0..points.len() {
                    if visited[i] {
                        continue;
                    }
                    let distance: i32 = (points[current][0] - points[i][0]).abs()
                        + (points[current][1] - points[i][1]).abs();

                    min_distance[i] = min_distance[i].min(distance);

                    if next == points.len() || min_distance[i] <= min_distance[next] {
                        next = i;
                    }
                }
                current = next;
                result += min_distance[next];
                count += 1;
            }

            result
        }
    }
}
pub mod prims {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    impl Solution {
        pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
            let mut n: usize = points.len();
            let mut adj: Vec<Vec<(i32, usize)>> = vec![vec![]; n];
            for i in 0..n {
                for j in i + 1..n {
                    let distance: i32 =
                        (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                    adj[i].push((distance, j));
                    adj[j].push((distance, i));
                }
            }

            let mut heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
            let mut visited: Vec<bool> = vec![false; n];
            let mut result: i32 = 0;
            heap.push((Reverse(0), 0));
            while let Some((Reverse(distance), index)) = heap.pop() {
                if visited[index] {
                    continue;
                }
                visited[index] = true;
                n -= 1;
                result += distance;
                if n == 0 {
                    return result;
                }
                for (dist, next) in adj[index].iter() {
                    if visited[*next] {
                        continue;
                    }
                    heap.push((Reverse(*dist), *next));
                }
            }

            -1i32
        }
    }
}
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
