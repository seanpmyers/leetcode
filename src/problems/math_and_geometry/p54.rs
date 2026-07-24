pub mod first {
    pub struct Solution;
    impl Solution {
        pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
            let rows: usize = matrix.len();
            let columns: usize = matrix[0].len();
            let mut result: Vec<i32> = vec![-101i32; rows * columns];

            let directions: [(isize, isize); 4] = [
                (0, 1),  //right
                (1, 0),  //down
                (0, -1), //left
                (-1, 0), //up
            ];
            let mut direction: usize = 0;
            let mut queue: Vec<(usize, usize)> = vec![(0, 0)];
            let mut i: usize = 0;

            while let Some((r, c)) = queue.pop() {
                result[i] = matrix[r][c];
                matrix[r][c] = 101;
                i += 1;

                let (dr, dc) = directions[direction];

                if let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc))
                    && nr < rows
                    && nc < columns
                    && matrix[nr][nc] != 101
                {
                    queue.push((nr, nc));
                    continue;
                }

                direction += 1;
                direction %= 4;

                let (dr, dc) = directions[direction];

                if let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc))
                    && nr < rows
                    && nc < columns
                    && matrix[nr][nc] != 101
                {
                    queue.push((nr, nc));
                }
            }

            result
        }
    }
}
