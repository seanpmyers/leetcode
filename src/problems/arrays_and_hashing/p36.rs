pub mod array {
    pub struct Solution;
    impl Solution {
        pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
            let mut rows: [[bool; 9usize]; 9usize] = [[false; 9usize]; 9usize];
            let mut columns: [[bool; 9usize]; 9usize] = [[false; 9usize]; 9usize];
            let mut boxes: [[bool; 9usize]; 9usize] = [[false; 9usize]; 9usize];

            for r in 0..9 {
                for c in 0..9 {
                    if board[r][c] == '.' {
                        continue;
                    }
                    let x: usize = board[r][c].to_digit(10).unwrap() as usize - 1;
                    let bi: usize = 3 * (r / 3) + (c / 3);
                    if rows[r][x] {
                        return false;
                    }
                    if columns[c][x] {
                        return false;
                    }
                    if boxes[bi][x] {
                        return false;
                    }
                    rows[r][x] = true;
                    columns[c][x] = true;
                    boxes[bi][x] = true;
                }
            }

            true
        }
    }
}
// #arrays #hashing
pub mod hashset {
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
}
