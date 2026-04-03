pub mod kahns {
    pub struct Solution;
    use std::collections::VecDeque;
    impl Solution {
        pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
            let mut list: [i32; 2_000usize] = [0i32; 2_000usize];
            let mut reqs: Vec<Vec<usize>> = vec![vec![]; num_courses as usize];
            for i in 0..prerequisites.len() {
                let course = &prerequisites[i];
                list[course[1] as usize] += 1;
                reqs[course[0] as usize].push(course[1] as usize);
            }

            let mut queue: VecDeque<usize> = VecDeque::new();
            let n: usize = num_courses as usize;
            for c in 0..n {
                if list[c] > 0 {
                    continue;
                }
                queue.push_back(c);
            }

            let mut count: usize = 0usize;
            while let Some(c) = queue.pop_front() {
                count += 1;
                for &x in &reqs[c] {
                    list[x] -= 1;
                    if list[x] == 0 {
                        queue.push_back(x);
                    }
                }
            }

            count == n
        }
    }
}

pub mod optimized {
    pub struct Solution {}
    use std::collections::{HashMap, HashSet, VecDeque};
    impl Solution {
        pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
            let rows = prerequisites.len();

            let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

            let mut queue: VecDeque<usize> = VecDeque::new();
            for r in 0..rows {
                queue.push_back(r);
                let course = prerequisites[r][0];
                let prereq = prerequisites[r][1];
                map.entry(course)
                    .and_modify(|x| x.push(prereq))
                    .or_insert(vec![prereq]);
            }

            let mut visited: HashSet<i32> = HashSet::new();
            for course in 0..num_courses {
                if !Self::dfs(course, &mut map, &mut visited) {
                    return false;
                };
            }
            true
        }

        pub fn dfs(
            course: i32,
            map: &mut HashMap<i32, Vec<i32>>,
            visited: &mut HashSet<i32>,
        ) -> bool {
            if visited.contains(&course) {
                return false;
            }

            if map.get(&course).is_some_and(|prereqs| prereqs.is_empty()) {
                return true;
            }

            visited.insert(course);

            let current: Vec<i32> = map.get(&course).unwrap_or(&vec![]).clone();

            for prereq in current.into_iter() {
                if !Self::dfs(prereq, map, visited) {
                    return false;
                }
            }

            visited.remove(&course);
            map.entry(course).and_modify(|x| x.clear());

            true
        }
    }
}

pub mod brute_force {
    pub struct Solution {}
    use std::collections::{HashSet, VecDeque};
    impl Solution {
        pub fn can_finish(_num_courses: i32, mut prerequisites: Vec<Vec<i32>>) -> bool {
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

pub mod almost_kahns {
    pub struct Solution;
    impl Solution {
        pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
            let mut list: [bool; 2_000usize] = [true; 2_000usize];
            let mut reqs: Vec<Vec<usize>> = vec![vec![]; num_courses as usize];
            for i in 0..prerequisites.len() {
                let course = &prerequisites[i];
                list[course[0] as usize] = false;
                reqs[course[0] as usize].push(course[1] as usize);
            }

            let mut stack = vec![];
            let n: usize = num_courses as usize;
            for c in 0..n {
                if !list[c] {
                    continue;
                }
                stack.push(c);
            }

            while let Some(c) = stack.pop() {
                for r in 0..reqs.len() {
                    if reqs[r].is_empty() {
                        continue;
                    }
                    reqs[r] = reqs[r]
                        .iter()
                        .filter(|x| **x != c)
                        .cloned()
                        .collect::<Vec<usize>>();

                    if reqs[r].is_empty() {
                        stack.push(r);
                    }
                }
            }

            reqs.iter().all(|x| x.is_empty())
        }
    }
}
