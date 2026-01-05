pub struct Solution;
use std::collections::VecDeque;
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut result: i32 = 0;

        let rows: usize = grid.len();
        let columns: usize = grid[0].len();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut fresh: u32 = 0;

        for r in 0..rows {
            for c in 0..columns {
                match grid[r][c] {
                    1 => {
                        fresh = fresh.saturating_add(1);
                    }
                    2 => queue.push_back((r, c)),
                    _ => {}
                }
            }
        }

        if fresh == 0 {
            return 0i32;
        }

        let directions: [(isize, isize); 4] = [
            (-1isize, 0isize),
            (1isize, 0isize),
            (0isize, -1isize),
            (0isize, 1isize),
        ];

        while fresh > 0 && !queue.is_empty() {
            let length: usize = queue.len();
            for _ in 0..length {
                let Some((r, c)) = queue.pop_front() else {
                    continue;
                };

                for (dr, dc) in directions.iter() {
                    let (Some(r), Some(c)) = (r.checked_add_signed(*dr), c.checked_add_signed(*dc))
                    else {
                        continue;
                    };

                    if r >= rows || c >= columns || grid[r][c] != 1 {
                        continue;
                    }

                    grid[r][c] = 2;
                    queue.push_back((r, c));
                    fresh = fresh.saturating_sub(1);
                }
            }
            result = result.saturating_add(1);
        }

        if fresh > 0 {
            return -1;
        }

        result
    }
}
