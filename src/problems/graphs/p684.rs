pub mod dfs {
    pub struct Solution;
    impl Solution {
        pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
            let n: usize = edges.len();
            if n > 10usize.pow(3u32) || n < 3usize {
                panic!("Constraint");
            }

            let mut adjacent: Vec<Vec<i32>> = vec![Vec::new(); n + 1];

            for pair in edges.iter() {
                let (u, v): (i32, i32) = (pair[0], pair[1]);

                adjacent[u as usize].push(v);
                adjacent[v as usize].push(u);
                let mut visit: Vec<bool> = vec![false; n + 1];
                if Self::dfs(u, -1, &adjacent, &mut visit) {
                    return vec![u, v];
                }
            }

            vec![]
        }

        pub fn dfs(
            node: i32,
            parent: i32,
            adjacent: &Vec<Vec<i32>>,
            visit: &mut Vec<bool>,
        ) -> bool {
            if visit[node as usize] {
                return true;
            }

            visit[node as usize] = true;
            for neighbor in adjacent[node as usize].iter() {
                if *neighbor == parent {
                    continue;
                }

                if Self::dfs(*neighbor, node, adjacent, visit) {
                    return true;
                }
            }

            false
        }
    }
}
