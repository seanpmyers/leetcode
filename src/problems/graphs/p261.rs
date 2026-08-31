pub mod stack {
    pub struct Solution;
    impl Solution {
        pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
            if edges.is_empty() {
                return n <= 1;
            }
            let mut n: usize = n as usize;
            let mut adj: Vec<Vec<usize>> = vec![vec![]; n];

            for edge in edges.iter() {
                let x: usize = edge[0] as usize;
                let y: usize = edge[1] as usize;
                adj[x].push(y);
                adj[y].push(x);
            }

            let mut visited: Vec<bool> = vec![false; n];
            let mut stack: Vec<usize> = vec![edges[0][0] as usize];

            while let Some(node) = stack.pop() {
                if visited[node] {
                    return false;
                }
                n -= 1;
                visited[node] = true;
                for &x in adj[node].iter() {
                    if visited[x] {
                        continue;
                    }
                    stack.push(x);
                }
            }

            n == 0
        }
    }
}
pub mod dfs {
    pub struct Solution;
    use std::collections::{HashMap, HashSet};
    impl Solution {
        pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
            let n = n as usize;
            if edges.len() > n - 1 || edges.len() < n - 1 {
                return false;
            }
            if n == 1 {
                return true;
            }

            let rows: usize = edges.len();
            let mut map: HashMap<i32, Vec<i32>> = HashMap::with_capacity(n);
            for r in 0..rows {
                map.entry(edges[r][0]).or_default().push(edges[r][1]);
                map.entry(edges[r][1]).or_default().push(edges[r][0]);
            }

            let mut visited: HashSet<i32> = HashSet::new();

            Self::dfs(&mut visited, &map, edges[0][0].clone(), -1) && visited.len() == n
        }

        pub fn dfs(
            visited: &mut HashSet<i32>,
            map: &HashMap<i32, Vec<i32>>,
            node: i32,
            parent: i32,
        ) -> bool {
            if !visited.insert(node) {
                return false;
            }

            let Some(list) = map.get(&node) else {
                return false;
            };

            for n in list.iter() {
                if *n == parent {
                    continue;
                }
                if !Self::dfs(visited, map, *n, node) {
                    return false;
                }
            }

            true
        }
    }
}
