pub mod first_attempt {
    pub struct Solution;
    impl Solution {
        pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
            let rows: usize = matrix.len();
            let columns: usize = matrix[0].len();
            let mut dp: Vec<Vec<i32>> = vec![vec![-1i32; columns]; rows];
            let mut result: i32 = -1i32;
            let directions: [(isize, isize); 4usize] = [
                (0isize, -1isize), // up
                (0isize, 1isize),  // down
                (-1isize, 0isize), // left
                (1isize, 0isize),  // right
            ];
            fn dfs(
                r: usize,
                c: usize,
                rows: usize,
                columns: usize,
                directions: &[(isize, isize); 4usize],
                matrix: &Vec<Vec<i32>>,
                dp: &mut Vec<Vec<i32>>,
            ) -> i32 {
                let mut result: i32 = 1;
                for d in directions {
                    let (Some(dr), Some(dc)) =
                        (r.checked_add_signed(d.0), c.checked_add_signed(d.1))
                    else {
                        continue;
                    };
                    if dr >= rows || dc >= columns {
                        continue;
                    }
                    if matrix[dr][dc] <= matrix[r][c] {
                        continue;
                    }
                    if dp[dr][dc] != -1i32 {
                        result = result.max(1 + dp[dr][dc]);
                        continue;
                    }
                    result = result.max(1 + dfs(dr, dc, rows, columns, directions, matrix, dp));
                }
                dp[r][c] = result;
                result
            }

            for r in 0..rows {
                for c in 0..columns {
                    if dp[r][c] != -1i32 {
                        continue;
                    }
                    result = result.max(dfs(r, c, rows, columns, &directions, &matrix, &mut dp));
                }
            }

            result
        }
    }
}
