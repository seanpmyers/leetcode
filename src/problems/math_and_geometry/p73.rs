pub struct Solution;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let rows: usize = matrix.len();
        let columns: usize = matrix[0].len();
        let mut row_zero = false;

        for r in 0..rows {
            for c in 0..columns {
                if matrix[r][c] == 0 {
                    matrix[0][c] = 0;
                    if r > 0 {
                        matrix[r][0] = 0;
                    } else {
                        row_zero = true;
                    }
                }
            }
        }

        for r in 1..rows {
            for c in 1..columns {
                if matrix[0][c] == 0 || matrix[r][0] == 0 {
                    matrix[r][c] = 0;
                }
            }
        }

        if matrix[0][0] == 0 {
            for r in 0..rows {
                matrix[r][0] = 0;
            }
        }

        if row_zero {
            for c in 0..columns {
                matrix[0][c] = 0;
            }
        }
    }
}
