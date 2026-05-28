pub mod first_attempt {
    pub struct Solution;
    impl Solution {
        pub fn island_perimeter(mut grid: Vec<Vec<i32>>) -> i32 {
            let mut stack: Vec<(usize, usize)> = Vec::new();
            let mut result: i32 = 0;
            let directions: [(isize, isize); 4] = [
                (1, 0),  //down
                (-1, 0), //up
                (0, -1), //left
                (0, 1),  //right
            ];
            let rows = grid.len();
            let columns = grid[0].len();

            'outer: for r in 0..rows {
                for c in 0..columns {
                    if grid[r][c] != 1 {
                        continue;
                    }
                    stack.push((r, c));
                    break 'outer;
                }
            }

            while let Some((r, c)) = stack.pop() {
                if grid[r][c] == -1 {
                    continue;
                }
                grid[r][c] = -1;
                // println!("current {r} {c}");
                for d in directions.iter() {
                    let (Some(dr), Some(dc)) =
                        (r.checked_add_signed(d.0), c.checked_add_signed(d.1))
                    else {
                        // println!("left or up");
                        result += 1;
                        continue;
                    };
                    if dr >= rows || dc >= columns {
                        // println!("down or right {dr} {dc}");
                        result += 1;
                        continue;
                    }
                    //visited
                    if grid[dr][dc] == -1i32 {
                        continue;
                    }
                    //water
                    if grid[dr][dc] == 0 {
                        // println!("water");
                        result += 1;
                        continue;
                    }
                    stack.push((dr, dc));
                }
            }

            result
        }
    }
}
