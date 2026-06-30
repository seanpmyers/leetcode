pub mod reverse_transpose {
    pub struct Solution;
    impl Solution {
        pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
            matrix.reverse();
            let rows = matrix.len();
            let columns = matrix[0].len();
            for r in 0..rows {
                for c in r + 1..columns {
                    let temp = matrix[r][c];
                    matrix[r][c] = matrix[c][r];
                    matrix[c][r] = temp;
                }
            }
        }
    }
}
pub mod swap_four {
    pub struct Solution;
    impl Solution {
        pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
            let mut l: usize = 0;
            let mut r: usize = matrix.len() - 1;

            while l < r {
                for i in 0..(r - l) {
                    let top: usize = l;
                    let bottom: usize = r;
                    let top_left = (top, l + i);
                    let bottom_left = (bottom - i, l);
                    let bottom_right = (bottom, r - i);
                    let top_right = (top + i, r);
                    let tl = matrix[top_left.0][top_left.1];
                    matrix[top_left.0][top_left.1] = matrix[bottom_left.0][bottom_left.1];
                    matrix[bottom_left.0][bottom_left.1] = matrix[bottom_right.0][bottom_right.1];
                    matrix[bottom_right.0][bottom_right.1] = matrix[top_right.0][top_right.1];
                    matrix[top_right.0][top_right.1] = tl;
                }
                r -= 1;
                l += 1;
            }
        }
    }
}
