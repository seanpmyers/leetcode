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
        let directions: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        queue.push_back((row, column));

        while let Some((row, column)) = queue.pop_front() {
            for (r, c) in directions.iter() {
                let (x, y): (i32, i32) = (
                    (row as i32).saturating_add(*r),
                    (column as i32).saturating_add(*c),
                );
                if x < 0
                    || y < 0
                    || x >= rows as i32
                    || y >= columns as i32
                    || grid[x as usize][y as usize] == '0'
                {
                    continue;
                }

                grid[x as usize][y as usize] = '0';
                queue.push_back((x as usize, y as usize));
            }
        }
    }
}
