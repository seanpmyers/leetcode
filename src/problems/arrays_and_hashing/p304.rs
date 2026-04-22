#[allow(dead_code)]
pub mod inclusion_exclusion_principle {
    struct NumMatrix {
        pub m: Vec<Vec<i32>>,
    }

    impl NumMatrix {
        fn new(matrix: Vec<Vec<i32>>) -> Self {
            let rows: usize = matrix.len();
            let cols: usize = matrix[0].len();
            let mut m: Vec<Vec<i32>> = vec![vec![0; cols + 1]; rows + 1];

            for r in 0..rows {
                let mut prefix: i32 = 0;
                for c in 0..cols {
                    prefix += matrix[r][c];
                    m[r + 1][c + 1] = prefix;
                    m[r + 1][c + 1] += m[r][c + 1];
                }
            }

            Self { m }
        }

        fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
            let row1: usize = (row1 + 1i32) as usize;
            let col1: usize = (col1 + 1i32) as usize;
            let row2: usize = (row2 + 1i32) as usize;
            let col2: usize = (col2 + 1i32) as usize;

            let bottom_right: i32 = self.m[row2][col2];

            let bottom_left: i32 = self.m[row2][col1 - 1];
            let top_right: i32 = self.m[row1 - 1][col2];
            let top_left: i32 = self.m[row1 - 1][col1 - 1];

            bottom_right - bottom_left - top_right + top_left
        }
    }
}

#[allow(dead_code)]
pub mod looped_sum {
    struct NumMatrix {
        pub matrix: Vec<Vec<i32>>,
    }

    impl NumMatrix {
        fn new(matrix: Vec<Vec<i32>>) -> Self {
            let mut m: Vec<Vec<i32>> = vec![vec![0i32; matrix[0].len()]; matrix.len()];
            for r in 0..matrix.len() {
                for c in 0..matrix[0].len() {
                    let previous: i32 = if c == 0 && r == 0 {
                        0i32
                    } else {
                        if c == 0 {
                            m[r - 1][matrix[0].len() - 1]
                        } else {
                            m[r][c - 1]
                        }
                    };
                    m[r][c] = previous + matrix[r][c];
                }
            }
            Self { matrix: m }
        }

        fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
            let row1 = row1 as usize;
            let row2 = row2 as usize;
            let col1 = col1 as usize;
            let col2 = col2 as usize;

            let mut result: i32 = 0i32;
            for r in row1..=row2 {
                let previous = if col1 == 0 && r == 0 {
                    0i32
                } else {
                    if col1 == 0 {
                        self.matrix[r - 1][self.matrix[0].len() - 1]
                    } else {
                        self.matrix[r][col1 - 1]
                    }
                };
                result += self.matrix[r][col2] - previous;
            }

            result
        }
    }
}
