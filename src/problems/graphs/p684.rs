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

pub mod disjoint_set_union_2 {
    pub struct Solution;
    pub struct UnionFind {
        pub parents: Vec<usize>,
        pub rank: Vec<usize>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self {
                parents: (0..n + 1).collect(),
                rank: vec![0; n + 1],
            }
        }

        pub fn find(&mut self, n: usize) -> usize {
            let mut p: usize = self.parents[n];

            while p != self.parents[p] {
                self.parents[p] = self.parents[self.parents[p]];
                p = self.parents[p];
            }

            p
        }

        pub fn union(&mut self, x: usize, y: usize) -> bool {
            let x: usize = self.find(x);
            let y: usize = self.find(y);

            if x == y {
                return false;
            }

            match self.rank[x].cmp(&self.rank[y]) {
                std::cmp::Ordering::Greater => {
                    self.parents[y] = x;
                }
                std::cmp::Ordering::Less => {
                    self.parents[x] = y;
                }
                std::cmp::Ordering::Equal => {
                    self.parents[y] = x;
                    self.rank[x] += self.rank[y];
                }
            }

            true
        }
    }

    impl Solution {
        pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
            let mut find: UnionFind = UnionFind::new(edges.len());
            let mut result: Vec<i32> = vec![0i32; 2usize];

            for i in 0..edges.len() {
                let x = edges[i][0];
                let y = edges[i][1];
                if !find.union(x as usize, y as usize) {
                    result = vec![x, y];
                }
            }

            result
        }
    }
}

pub mod disjoint_set_union {
    pub struct Solution;
    impl Solution {
        pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
            let mut parents: Vec<usize> = (0..edges.len()).collect();
            let mut ranks: Vec<i32> = vec![1; edges.len()];
            let mut result: Vec<i32> = Vec::new();
            for edge in &edges {
                if !Self::union(
                    edge[0] as usize - 1,
                    edge[1] as usize - 1,
                    &mut parents,
                    &mut ranks,
                ) {
                    result = edge.clone()
                }
            }

            result
        }

        pub fn find(x: usize, parents: &mut Vec<usize>) -> usize {
            let mut parent: usize = x;
            while parent != parents[parent] {
                parents[parent] = parents[parents[parent]];
                parent = parents[parent];
            }

            parent
        }

        pub fn union(x: usize, y: usize, parents: &mut Vec<usize>, ranks: &mut Vec<i32>) -> bool {
            let x: usize = Self::find(x, parents);
            let y: usize = Self::find(y, parents);

            if x == y {
                return false;
            }

            match ranks[x].cmp(&ranks[y]) {
                std::cmp::Ordering::Less => parents[x] = y,
                std::cmp::Ordering::Greater => parents[y] = x,
                std::cmp::Ordering::Equal => {
                    ranks[x] = ranks[y] + 1;
                    parents[x] = y;
                }
            }

            true
        }
    }
}
