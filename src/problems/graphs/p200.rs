pub struct Solution;
use std::collections::VecDeque;
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut result: i32 = 0;

        let rows: usize = grid.len();
        let columns: usize = grid[0].len();

        for row in 0..rows {
            for column in 0..columns {
                match grid[row][column] {
                    '1' => {
                        Self::bfs(&mut grid, (row, column));
                        result = result.saturating_add(1);
                    }
                    '0' | _ => {}
                }
            }
        }

        result
    }

    pub fn bfs(grid: &mut Vec<Vec<char>>, (row, column): (usize, usize)) {
        let rows: usize = grid.len();
        let columns: usize = grid[0].len();

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let directions: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        queue.push_back((row, column));

        while let Some((row, column)) = queue.pop_front() {
            for (dr, dc) in directions.iter() {
                if let (Some(r), Some(c)) =
                    (row.checked_add_signed(*dr), column.checked_add_signed(*dc))
                {
                    if r >= rows || c >= columns || grid[r][c] == '0' {
                        continue;
                    }

                    grid[r][c] = '0';
                    queue.push_back((r, c));
                }
            }
        }
    }
}
