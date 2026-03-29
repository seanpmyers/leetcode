pub mod bfs {
    pub struct Solution;
    use std::collections::VecDeque;
    impl Solution {
        pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
            let rows: usize = grid.len();
            let columns: usize = grid[0].len();

            let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::with_capacity(rows);

            let mut result: i32 = i32::MAX;
            queue.push_back((0usize, 0usize, 1i32));
            let directions: [(isize, isize); 8] = [
                (0, 1),   //right
                (1, 1),   //down right
                (1, 0),   //down
                (-1, 1),  //up right
                (-1, 0),  //up
                (0, -1),  //left
                (1, -1),  //down left
                (-1, -1), //up left
            ];
            while let Some((r, c, v)) = queue.pop_front() {
                if grid[r][c] == 1 {
                    continue;
                }
                grid[r][c] = 1;

                if r == rows - 1 && c == columns - 1 {
                    result = result.min(v);
                    return result;
                }
                let v = v + 1;

                for &d in &directions {
                    if let (Some(dr), Some(dc)) =
                        (r.checked_add_signed(d.0), c.checked_add_signed(d.1))
                    {
                        if dr < rows && dc < columns {
                            queue.push_back((dr, dc, v));
                        }
                    }
                }
            }

            -1i32
        }
    }
}
