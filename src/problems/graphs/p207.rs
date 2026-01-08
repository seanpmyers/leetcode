pub mod brute_force {
    pub struct Solution {}
    use std::collections::{HashSet, VecDeque};
    impl Solution {
        pub fn can_finish(num_courses: i32, mut prerequisites: Vec<Vec<i32>>) -> bool {
            let rows = prerequisites.len();

            let mut queue: VecDeque<usize> = VecDeque::new();
            for r in 0..rows {
                queue.push_back(r);
            }

            let mut visited: HashSet<usize> = HashSet::new();

            'outer: while let Some(row) = queue.pop_front() {
                if prerequisites[row][0] == prerequisites[row][1] {
                    return false;
                }
                // println!("LOOP: row {row}");
                let prereq: i32 = prerequisites[row][1];

                if visited.contains(&row) {
                    // println!("VISITED: row {row}");
                    return false;
                }

                for rc in queue.iter() {
                    // println!("INNER: {row} {rc}");
                    if prerequisites[*rc][0] == prereq {
                        // println!("PREREQ: {row} {rc}");
                        visited.insert(row);
                        queue.push_back(row);
                        continue 'outer;
                    }
                }
                // println!("CLEARED: row {row}");
                visited.clear();
                prerequisites[row][1] = -1;
            }

            true
        }
    }
}
