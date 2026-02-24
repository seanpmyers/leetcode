pub mod bfs {
    pub struct Solution;
    use std::collections::VecDeque;
    impl Solution {
        pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
            let rows: usize = rooms.len();
            let columns: usize = rooms[0].len();
            let mut gates: Vec<(usize, usize)> = vec![];
            for r in 0..rows {
                for c in 0..columns {
                    match rooms[r][c] {
                        0 => gates.push((r, c)),
                        _ => {}
                    }
                }
            }
            let mut bfs: VecDeque<(usize, usize, i32)> = VecDeque::new();
            for (r, c) in gates.into_iter() {
                if r != 0 {
                    bfs.push_back((r - 1, c, 1));
                }
                if c != 0 {
                    bfs.push_back((r, c - 1, 1));
                }
                if r + 1 < rows {
                    bfs.push_back((r + 1, c, 1));
                }
                if c + 1 < columns {
                    bfs.push_back((r, c + 1, 1));
                }
            }

            while let Some((r, c, steps)) = bfs.pop_front() {
                let room: i32 = rooms[r][c];
                if room == -1 || room == 0 {
                    continue;
                }
                if room < steps {
                    continue;
                }
                rooms[r][c] = room.min(steps);
                if r != 0 {
                    bfs.push_back((r - 1, c, steps + 1));
                }
                if c != 0 {
                    bfs.push_back((r, c - 1, steps + 1));
                }
                if r + 1 < rows {
                    bfs.push_back((r + 1, c, steps + 1));
                }
                if c + 1 < columns {
                    bfs.push_back((r, c + 1, steps + 1));
                }
            }
        }
    }
}
