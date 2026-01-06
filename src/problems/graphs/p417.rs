pub mod bfs {
    pub struct Solution;
    use std::collections::VecDeque;
    impl Solution {
        pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = Vec::with_capacity(heights.len());

            let rows: usize = heights.len();
            let columns: usize = heights[0].len();

            let mut atlantic: Vec<Vec<bool>> = vec![vec![false; columns]; rows];
            let mut pacific: Vec<Vec<bool>> = vec![vec![false; columns]; rows];

            let mut atlantic_queue: VecDeque<(usize, usize)> = VecDeque::new();
            let mut pacific_queue: VecDeque<(usize, usize)> = VecDeque::new();

            for c in 0..columns {
                pacific_queue.push_back((0, c));
                atlantic_queue.push_back((rows.saturating_sub(1), c));
            }
            for r in 0..rows {
                pacific_queue.push_back((r, 0));
                atlantic_queue.push_back((r, columns.saturating_sub(1)));
            }

            Self::bfs(&heights, &mut pacific, &mut pacific_queue);
            Self::bfs(&heights, &mut atlantic, &mut atlantic_queue);

            for r in 0..rows {
                for c in 0..columns {
                    if pacific[r][c] && atlantic[r][c] {
                        result.push(vec![r as i32, c as i32]);
                    }
                }
            }

            result
        }

        pub fn bfs(
            heights: &Vec<Vec<i32>>,
            ocean: &mut Vec<Vec<bool>>,
            queue: &mut VecDeque<(usize, usize)>,
        ) {
            let rows: usize = heights.len();
            let columns: usize = heights[0].len();

            let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            while let Some((r, c)) = queue.pop_front() {
                ocean[r][c] = true;
                for (dr, dc) in directions.iter() {
                    let (Some(nr), Some(nc)) =
                        (r.checked_add_signed(*dr), c.checked_add_signed(*dc))
                    else {
                        continue;
                    };

                    if nr >= rows
                        || nc >= columns
                        || heights[r][c] > heights[nr][nc]
                        || ocean[nr][nc]
                    {
                        continue;
                    }

                    queue.push_back((nr, nc));
                }
            }
        }
    }
}

pub mod first_attempt {
    pub struct Solution;
    use std::collections::{HashMap, VecDeque};
    impl Solution {
        pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = Vec::with_capacity(heights.len());
            let mut visited: HashMap<(usize, usize), bool> = HashMap::new();

            let rows: usize = heights.len();
            let columns: usize = heights[0].len();

            for r in 0..rows {
                for c in 0..columns {
                    let can_flow: bool = Self::bfs(&heights, (r, c), &visited);
                    visited.insert((r, c), can_flow);
                    if can_flow {
                        result.push(vec![r as i32, c as i32]);
                    }
                }
            }

            result
        }

        pub fn bfs(
            heights: &Vec<Vec<i32>>,
            (row, column): (usize, usize),
            visited: &HashMap<(usize, usize), bool>,
        ) -> bool {
            let rows: usize = heights.len();
            let columns: usize = heights[0].len();

            let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::with_capacity(heights.len());
            // let mut visited: HashSet<(usize,usize)>
            let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut pacific: bool = false;
            let mut atlantic: bool = false;

            queue.push_back((row, column, heights[row][column]));

            while let Some((r, c, height)) = queue.pop_front() {
                for (dr, dc) in directions.iter() {
                    if pacific && atlantic {
                        return true;
                    }
                    match (r.checked_add_signed(*dr), c.checked_add_signed(*dc)) {
                        (None, None) | (None, Some(_)) | (Some(_), None) => {
                            pacific = true;
                        }
                        (Some(r), Some(c)) => {
                            if r >= rows || c >= columns {
                                atlantic = true;
                                continue;
                            }

                            if heights[r][c] > height || visited.get(&(r, c)).is_some_and(|x| !*x) {
                                continue;
                            }

                            queue.push_back((r, c, heights[r][c]));
                        }
                    }
                }
            }

            pacific && atlantic
        }
    }
}
