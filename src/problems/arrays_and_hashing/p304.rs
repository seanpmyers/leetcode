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
