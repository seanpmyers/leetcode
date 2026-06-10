pub mod topological {
    pub struct Solution;
    use std::collections::VecDeque;
    impl Solution {
        pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
            let mut n: usize = num_courses as usize;
            let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
            let mut indegree: Vec<u32> = vec![0u32; n + 1];
            for pair in prerequisites.into_iter() {
                let req = pair[1] as usize;
                let course = pair[0] as usize;
                adj[req].push(course);
                indegree[course] += 1;
            }

            let mut queue: VecDeque<usize> = VecDeque::with_capacity(n);
            let mut result: Vec<i32> = vec![];

            for i in 0..n {
                if indegree[i] > 0 {
                    continue;
                }
                queue.push_back(i);
            }

            while let Some(req) = queue.pop_front() {
                result.push(req as i32);
                for i in 0..adj[req].len() {
                    let course = adj[req][i];
                    indegree[course] -= 1;
                    if indegree[course] == 0 {
                        queue.push_back(course);
                    }
                }
            }

            if result.len() < n {
                return vec![];
            }

            result
        }
    }
}
pub mod dfs {
    pub struct Solution;
    use std::collections::{HashMap, HashSet};
    impl Solution {
        pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
            let mut result: Vec<i32> = Vec::with_capacity(num_courses as usize);

            let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
            let mut visited: HashSet<i32> = HashSet::new();

            for i in 0..prerequisites.len() {
                let course: i32 = prerequisites[i][0];
                let prerequisite: i32 = prerequisites[i][1];
                if course == prerequisite {
                    return vec![];
                }

                map.entry(course)
                    .and_modify(|prereqs| prereqs.push(prerequisite))
                    .or_insert(vec![prerequisite]);
            }

            for course in 0..num_courses {
                if !Self::dfs(&mut result, &mut map, &mut visited, course) {
                    return vec![];
                };
            }

            result
        }

        pub fn dfs(
            result: &mut Vec<i32>,
            map: &mut HashMap<i32, Vec<i32>>,
            visited: &mut HashSet<i32>,
            course: i32,
        ) -> bool {
            if visited.contains(&course) {
                return false;
            }

            if map.get(&course).is_some_and(|x| x.is_empty()) {
                return true;
            }

            visited.insert(course);

            let prereqs: Vec<i32> = map.get(&course).unwrap_or(&vec![]).clone();

            for req in prereqs.into_iter() {
                if !Self::dfs(result, map, visited, req) {
                    return false;
                }
            }

            visited.remove(&course);
            map.entry(course).and_modify(|x| x.clear());
            if !result.contains(&course) {
                result.push(course);
            }

            true
        }
    }
}
