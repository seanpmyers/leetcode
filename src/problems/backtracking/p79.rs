pub struct Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let rows = board.len();
        let cols = board[0].len();
        let word_chars: Vec<char> = word.chars().collect();

        for r in 0..rows {
            for c in 0..cols {
                if Self::dfs(&mut board, &word_chars, r as i32, c as i32, 0) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(board: &mut Vec<Vec<char>>, word: &[char], r: i32, c: i32, i: usize) -> bool {
        if i == word.len() {
            return true;
        }

        let rows = board.len() as i32;
        let cols = board[0].len() as i32;

        if r < 0 || c < 0 || r >= rows || c >= cols || board[r as usize][c as usize] != word[i] {
            return false;
        }

        let temp = board[r as usize][c as usize];
        board[r as usize][c as usize] = '#';

        let found = Self::dfs(board, word, r + 1, c, i + 1)
            || Self::dfs(board, word, r - 1, c, i + 1)
            || Self::dfs(board, word, r, c + 1, i + 1)
            || Self::dfs(board, word, r, c - 1, i + 1);

        board[r as usize][c as usize] = temp;

        found
    }
}
