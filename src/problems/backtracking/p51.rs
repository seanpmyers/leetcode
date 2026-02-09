pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n: usize = n as usize;
        let mut result: HashSet<Vec<String>> = HashSet::with_capacity(n);
        let board: Vec<Vec<char>> = vec![vec!['.'; n]; n];

        Self::backtrack(n, 0usize, board, &mut result);

        result.into_iter().collect()
    }

    pub fn backtrack(
        n: usize,
        queens: usize,
        board: Vec<Vec<char>>,
        result: &mut HashSet<Vec<String>>,
    ) {
        if queens == n {
            result.insert(Self::to_output(board.clone(), n));
            return;
        }

        for r in 0..n {
            for c in 0..n {
                if board[r][c] == 'x' || board[r][c] == 'Q' {
                    continue;
                }
                Self::backtrack(
                    n,
                    queens + 1,
                    Self::update_board(n, (r, c), board.clone()),
                    result,
                );
            }
        }
    }

    pub fn update_board(
        n: usize,
        (r, c): (usize, usize),
        mut board: Vec<Vec<char>>,
    ) -> Vec<Vec<char>> {
        board[r][c] = 'Q';

        for i in 0..n {
            if board[i][c] == '.' {
                board[i][c] = 'x';
            }
            if board[r][i] == '.' {
                board[r][i] = 'x';
            }
        }

        let up_right = (0..=r).rev().zip(c..n);
        let up_left = (0..=r).rev().zip((0..=c).rev());
        let down_right = (r..n).zip(c..n);
        let down_left = (r..n).zip((0..=c).rev());

        for (x, y) in up_left {
            if board[x][y] == '.' {
                board[x][y] = 'x';
            }
        }

        for (x, y) in up_right {
            if board[x][y] == '.' {
                board[x][y] = 'x';
            }
        }

        for (x, y) in down_left {
            if board[x][y] == '.' {
                board[x][y] = 'x';
            }
        }

        for (x, y) in down_right {
            if board[x][y] == '.' {
                board[x][y] = 'x';
            }
        }

        board
    }

    pub fn to_output(input: Vec<Vec<char>>, n: usize) -> Vec<String> {
        let mut result: Vec<String> = Vec::with_capacity(n);
        for r in 0..n {
            let mut row = String::with_capacity(n);
            for c in 0..n {
                if input[r][c] == 'Q' {
                    row.push('Q');
                    continue;
                }
                row.push('.');
            }
            result.push(row);
        }
        result
    }
}
