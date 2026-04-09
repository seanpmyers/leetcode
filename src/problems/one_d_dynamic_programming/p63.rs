pub mod iterative_dp_one_dimensional {
    pub struct Solution;
    impl Solution {
        pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
            let grid = obstacle_grid;
            if grid[0][0] == 1 {
                return 0i32;
            }
            let m: usize = grid.len();
            let n: usize = grid[0].len();
            if m == 1 && n == 1 {
                return if grid[0][0] == 1 { 0 } else { 1 };
            }

            let mut dp: Vec<i32> = vec![0; n];
            dp[0] = 1;

            for r in 0..m {
                for c in 0..n {
                    match grid[r][c] == 1 {
                        true => dp[c] = 0,
                        false => {
                            if c < 1 {
                                continue;
                            }
                            dp[c] += dp[c - 1];
                        }
                    }
                }
            }
            dp[n - 1]
        }
    }
}
pub mod iterative_dp {
    pub struct Solution;
    impl Solution {
        pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
            let grid = obstacle_grid;
            if grid[0][0] == 1 {
                return 0i32;
            }
            let m: usize = grid.len();
            let n: usize = grid[0].len();
            if m == 1 && n == 1 {
                return if grid[0][0] == 1 { 0 } else { 1 };
            }

            let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];
            dp[0][0] = 1;

            for r in 0..m {
                for c in 0..n {
                    if r == 0 && c == 0 {
                        continue;
                    }
                    match grid[r][c] == 1 {
                        true => dp[r][c] = 0,
                        false => {
                            let x = if r > 0 { dp[r - 1][c] } else { 0 };
                            let y = if c > 0 { dp[r][c - 1] } else { 0 };
                            dp[r][c] = x + y;
                        }
                    }
                }
            }
            dp[m - 1][n - 1]
        }
    }
}
pub mod bfs_dp {
    pub struct Solution;
    use std::collections::VecDeque;
    impl Solution {
        pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
            let grid = obstacle_grid;
            if grid[0][0] == 1 {
                return 0i32;
            }
            let m: usize = grid.len();
            let n: usize = grid[0].len();
            if m == 1 && n == 1 {
                return if grid[0][0] == 1 { 0 } else { 1 };
            }

            let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];
            dp[0][0] = 1;

            let directions: Vec<(usize, usize)> = vec![
                (1usize, 0usize), // down
                (0, 1),           // right
            ];

            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            if n > 1 && grid[0][1] != 1 {
                queue.push_back((0usize, 1usize));
            }
            if m > 1 && grid[1][0] != 1 {
                queue.push_back((1usize, 0usize));
            }
            while let Some((r, c)) = queue.pop_front() {
                if dp[r][c] != 0 {
                    continue;
                }
                let x: i32 = if r > 0 { dp[r - 1][c] } else { 0 };
                let y: i32 = if c > 0 { dp[r][c - 1] } else { 0 };
                dp[r][c] = x + y;
                for &d in &directions {
                    let dr: usize = r + d.0;
                    let dc: usize = c + d.1;
                    if dr >= m || dc >= n || grid[dr][dc] == 1 {
                        continue;
                    }
                    queue.push_back((dr, dc));
                }
            }

            dp[m - 1][n - 1]
        }
    }
}
