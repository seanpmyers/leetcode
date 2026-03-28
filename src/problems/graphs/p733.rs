pub struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let sr = sr as usize;
        let sc = sc as usize;
        let mut stack: Vec<(usize, usize)> = vec![(sr, sc)];
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let rows: usize = image.len();
        let columns: usize = image[0].len();
        let start: i32 = image[sr][sc];
        while let Some((r, c)) = stack.pop() {
            if !visited.insert((r, c)) || image[r][c] != start {
                continue;
            }

            image[r][c] = color;

            if r + 1 < rows {
                stack.push((r + 1, c));
            }

            if c + 1 < columns {
                stack.push((r, c + 1));
            }

            if let Some(row) = r.checked_sub(1) {
                stack.push((row, c));
            }

            if let Some(column) = c.checked_sub(1) {
                stack.push((r, column));
            }
        }

        image
    }
}
