pub struct Solution;
use std::collections::VecDeque;
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let rows = board.len();
        let columns = board[0].len();

        for r in 0..rows {
            match board[r][0] {
                'O' => {
                    Self::bfs(board, (r, 0));
                }
                'X' | _ => {}
            }
            match board[r][columns.saturating_sub(1)] {
                'O' => {
                    Self::bfs(board, (r, columns.saturating_sub(1)));
                }
                'X' | _ => {}
            }
        }

        for c in 0..columns {
            match board[0][c] {
                'O' => {
                    Self::bfs(board, (0, c));
                }
                'X' | _ => {}
            }
            match board[rows.saturating_sub(1)][c] {
                'O' => {
                    Self::bfs(board, (rows.saturating_sub(1), c));
                }
                'X' | _ => {}
            }
        }

        for r in 0..rows {
            for c in 0..columns {
                if board[r][c] == 'T' {
                    board[r][c] = 'O';
                    continue;
                }

                board[r][c] = 'X';
            }
        }
    }

    pub fn bfs(board: &mut Vec<Vec<char>>, (row, column): (usize, usize)) {
        let rows = board.len();
        let columns = board[0].len();
        let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

        queue.push_back((row, column));
        board[row][column] = 'T';

        while let Some((row, column)) = queue.pop_front() {
            for (dr, dc) in directions.iter() {
                let (Some(r), Some(c)) =
                    (row.checked_add_signed(*dr), column.checked_add_signed(*dc))
                else {
                    continue;
                };

                if r >= rows || c >= columns || board[r][c] == 'X' || board[r][c] == 'T' {
                    continue;
                }

                board[r][c] = 'T';
                queue.push_back((r, c));
            }
        }
    }
}
