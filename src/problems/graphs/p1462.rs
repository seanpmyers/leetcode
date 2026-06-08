pub mod dfs {
    pub struct Solution;
    impl Solution {
        pub fn check_if_prerequisite(
            num_courses: i32,
            prerequisites: Vec<Vec<i32>>,
            queries: Vec<Vec<i32>>,
        ) -> Vec<bool> {
            let n: usize = num_courses as usize;

            let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];

            for i in 0..prerequisites.len() {
                let req: usize = prerequisites[i][0] as usize;
                let course: usize = prerequisites[i][1] as usize;
                adj[course].push(req);
            }

            let mut result: Vec<bool> = vec![false; queries.len()];

            for i in 0..queries.len() {
                let req = queries[i][0] as usize;
                let course = queries[i][1] as usize;
                if result[i] {
                    continue;
                }
                let mut visited = vec![false; n + 1];
                result[i] = Self::search(course, req, &adj, &mut visited);
            }

            result
        }

        fn search(
            current: usize,
            target: usize,
            adj: &Vec<Vec<usize>>,
            visited: &mut Vec<bool>,
        ) -> bool {
            if visited[current] {
                return false;
            }
            visited[current] = true;
            if adj[current].is_empty() {
                return false;
            }

            for i in 0..adj[current].len() {
                if adj[current][i] == target {
                    return true;
                }

                if Self::search(adj[current][i], target, adj, visited) {
                    return true;
                }
            }

            false
        }
    }
}
