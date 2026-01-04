pub struct Solution;
use std::collections::VecDeque;
impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut result: i32 = 0;

        let rows: usize = grid.len();
        let columns: usize = grid[0].len();

        for r in 0..rows {
            for c in 0..columns {
                match grid[r][c] {
                    1 => {
                        let max = Self::bfs(&mut grid, (r, c));
                        result = result.max(max);
                    }
                    0 | _ => {}
                }
            }
        }

        result
    }

    pub fn bfs(grid: &mut Vec<Vec<i32>>, (row, column): (usize, usize)) -> i32 {
        let mut result: i32 = 0;
        let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let rows: usize = grid.len();
        let columns: usize = grid[0].len();

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

        queue.push_back((row, column));

        while let Some((row, column)) = queue.pop_front() {
            for (dr, dc) in directions.iter() {
                if let (Some(row), Some(column)) =
                    (row.checked_add_signed(*dr), column.checked_add_signed(*dc))
                {
                    if row >= rows || column >= columns || grid[row][column] == 0 {
                        continue;
                    }

                    grid[row][column] = 0;
                    result = result.saturating_add(1);
                    queue.push_back((row, column));
                }
            }
        }

        result.max(1)
    }
}
