pub mod backtrack {
    pub struct Solution;
    impl Solution {
        pub fn total_n_queens(n: i32) -> i32 {
            let n: usize = n as usize;
            let board: Vec<Vec<u8>> = vec![vec![b'.'; n]; n];
            let mut result: i32 = 0;
            Self::backtrack(board, &mut result, n, 0usize, 0usize);
            result
        }

        pub fn backtrack(
            board: Vec<Vec<u8>>,
            result: &mut i32,
            n: usize,
            row: usize,
            queens: usize,
        ) {
            if queens == n {
                *result += 1;
                return;
            }

            for c in 0..n {
                if board[row][c] == b'Q' || board[row][c] == b'x' {
                    continue;
                }
                Self::backtrack(
                    Self::update_board(row, c, n, board.clone()),
                    result,
                    n,
                    row + 1,
                    queens + 1,
                );
            }
        }

        pub fn update_board(r: usize, c: usize, n: usize, mut board: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
            board[r][c] = b'Q';

            for i in 0..n {
                if board[i][c] == b'.' {
                    board[i][c] = b'x';
                }
                if board[r][i] == b'.' {
                    board[r][i] = b'x';
                }
            }

            let down_right = (r..n).zip(c..n);
            let down_left = (r..n).zip((0..=c).rev());
            for (x, y) in down_left {
                if board[x][y] == b'.' {
                    board[x][y] = b'x';
                }
            }
            for (x, y) in down_right {
                if board[x][y] == b'.' {
                    board[x][y] = b'x';
                }
            }

            board
        }
    }
}
