// #arrays #hashing
pub struct Solution {}
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;
        let mut row: HashSet<(usize, u32)> = HashSet::new();
        let mut col: HashSet<(usize, u32)> = HashSet::new();
        let mut block: HashSet<(usize, u32)> = HashSet::new();
        for r in 0..board.len() {
            for c in 0..board[r].len() {
                if let Some(current) = board[r][c].to_digit(10) {
                    if !row.insert((r, current)) {
                        return false;
                    }
                    if !col.insert((c, current)) {
                        return false;
                    }
                    if !block.insert((3 * (r / 3) + (c / 3), current)) {
                        return false;
                    }
                }
            }
        }

        true
    }
}
