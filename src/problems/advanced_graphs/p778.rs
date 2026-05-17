pub mod dijkstra {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    impl Solution {
        pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
            let n: usize = grid.len();

            let directions: [(isize, isize); 4] = [
                (-1isize, 0isize), //up
                (0isize, -1isize), //left
                (0isize, 1isize),  //right
                (1isize, 0isize),  //down
            ];

            let mut heap: BinaryHeap<(Reverse<i32>, (usize, usize))> = BinaryHeap::new();
            let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; n];
            heap.push((Reverse(grid[0][0]), (0, 0)));

            while let Some((Reverse(depth), (r, c))) = heap.pop() {
                if r == n - 1 && c == n - 1 {
                    return depth;
                }
                if visited[r][c] {
                    continue;
                }
                visited[r][c] = true;

                for d in directions.iter() {
                    let (Some(dr), Some(dc)) =
                        (r.checked_add_signed(d.0), c.checked_add_signed(d.1))
                    else {
                        continue;
                    };
                    if dr >= n || dc >= n {
                        continue;
                    }
                    heap.push((Reverse(grid[dr][dc].max(depth)), (dr, dc)));
                }
            }

            -1i32
        }
    }
}
