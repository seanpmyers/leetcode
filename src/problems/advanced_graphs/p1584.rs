pub mod prims_practice {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    impl Solution {
        pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
            let mut n: usize = points.len();
            let mut list: Vec<Vec<(Reverse<i32>, usize)>> = vec![vec![]; n + 1];

            for i in 0..n {
                let x = &points[i];
                for j in i + 1..n {
                    let y = &points[j];
                    let dist: i32 = (x[0] - y[0]).abs() + (x[1] - y[1]).abs();
                    list[i].push((Reverse(dist), j));
                    list[j].push((Reverse(dist), i));
                }
            }

            let mut heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::with_capacity(n);
            heap.push((Reverse(0), 0));

            let mut result: i32 = 0;
            let mut visited: Vec<bool> = vec![false; n + 1];
            while n > 0
                && let Some((Reverse(dist), current)) = heap.pop()
            {
                if visited[current] {
                    continue;
                }
                visited[current] = true;
                result += dist;
                n -= 1;

                for point in list[current].iter() {
                    if visited[point.1] {
                        continue;
                    }
                    heap.push((point.0, point.1));
                }
            }

            result
        }
    }
}
pub mod heap_dsu {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    pub struct Dsu {
        pub parents: Vec<usize>,
        pub rank: Vec<usize>,
    }

    impl Dsu {
        pub fn new(n: usize) -> Self {
            Self {
                parents: (0..=n).collect(),
                rank: vec![1; n + 1],
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            let mut p = x;
            while self.parents[p] != p {
                self.parents[p] = self.parents[self.parents[p]];
                p = self.parents[p];
            }
            p
        }

        pub fn union(&mut self, x: usize, y: usize) -> bool {
            let mut x = self.find(x);
            let mut y = self.find(y);

            if x == y {
                return false;
            }

            if self.rank[x] < self.rank[y] {
                std::mem::swap(&mut x, &mut y);
            }

            self.rank[x] += self.rank[y];
            self.parents[y] = x;

            true
        }
    }

    impl Solution {
        pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
            let mut n: usize = points.len();

            let mut heap: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();

            for i in 0..n {
                let x = &points[i];
                for j in i + 1..n {
                    let y = &points[j];
                    let distance: i32 = (x[0] - y[0]).abs() + (x[1] - y[1]).abs();
                    heap.push((Reverse(distance), i, j));
                }
            }
            let mut dsu: Dsu = Dsu::new(n);
            let mut result: i32 = 0;

            while n > 0
                && let Some((Reverse(dist), x, y)) = heap.pop()
            {
                if !dsu.union(x, y) {
                    continue;
                }
                n -= 1;
                result += dist;
            }

            result
        }
    }
}
pub mod practice_kruskal {
    pub struct Solution;
    pub struct Dsu {
        pub parents: Vec<usize>,
        pub rank: Vec<usize>,
    }

    impl Dsu {
        pub fn new(n: usize) -> Self {
            Self {
                parents: (0..=n).collect(),
                rank: vec![1usize; n + 1],
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            let mut p: usize = x;

            while p != self.parents[p] {
                self.parents[p] = self.parents[self.parents[p]];
                p = self.parents[p];
            }

            p
        }

        pub fn union(&mut self, x: usize, y: usize) -> bool {
            let mut x: usize = self.find(x);
            let mut y: usize = self.find(y);

            if x == y {
                return false;
            }

            if self.rank[x] < self.rank[y] {
                std::mem::swap(&mut x, &mut y);
            }

            self.rank[x] += self.rank[y];
            self.parents[y] = x;

            true
        }
    }

    impl Solution {
        pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
            let n: usize = points.len();
            let mut dsu: Dsu = Dsu::new(n);
            let mut edges: Vec<(i32, usize, usize)> = vec![];
            for x in 0..n {
                for y in x + 1..n {
                    let distance: i32 =
                        (points[x][0] - points[y][0]).abs() + (points[x][1] - points[y][1]).abs();
                    edges.push((distance, x, y));
                }
            }

            edges.sort_unstable();

            let mut result: i32 = 0i32;
            for &(distance, x, y) in &edges {
                if !dsu.union(x, y) {
                    continue;
                }
                result += distance;
            }

            result
        }
    }
}
pub mod kruskals {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    impl Solution {
        pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
            let mut n: usize = points.len();
            let mut uf: UnionFind = UnionFind::new(n);
            let mut heap: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
            for i in 0..n {
                for j in i + 1..n {
                    let distance: i32 =
                        (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                    heap.push((Reverse(distance), i, j));
                }
            }
            let mut result: i32 = 0;
            while let Some((Reverse(distance), x, y)) = heap.pop() {
                if !uf.union(x, y) {
                    continue;
                }
                n -= 1;
                result += distance;
                if n == 1 {
                    break;
                }
            }
            result
        }
    }

    pub struct UnionFind {
        pub parents: Vec<usize>,
        pub ranks: Vec<usize>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self {
                parents: (0..n).collect(),
                ranks: vec![1; n],
            }
        }

        pub fn find(&mut self, i: usize) -> usize {
            let mut p: usize = i;
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

            if self.ranks[x] >= self.ranks[y] {
                self.ranks[x] += self.ranks[y];
                self.parents[y] = x;
                return true;
            }

            self.ranks[y] += self.ranks[x];
            self.parents[x] = y;

            true
        }
    }
}
pub mod prims_optimal {
    pub struct Solution;
    impl Solution {
        pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
            let n: usize = points.len();
            let mut result: i32 = 0;
            let mut current: usize = 0;
            let mut visited: Vec<bool> = vec![false; n];
            let mut min_distance: Vec<i32> = vec![10_000_000; n + 1];
            let mut count: usize = 0;

            while count < n - 1 {
                visited[current] = true;
                if n == 0 {
                    break;
                }
                let mut next: usize = points.len();
                for i in 0..points.len() {
                    if visited[i] {
                        continue;
                    }
                    let distance: i32 = (points[current][0] - points[i][0]).abs()
                        + (points[current][1] - points[i][1]).abs();

                    min_distance[i] = min_distance[i].min(distance);

                    if next == points.len() || min_distance[i] <= min_distance[next] {
                        next = i;
                    }
                }
                current = next;
                result += min_distance[next];
                count += 1;
            }

            result
        }
    }
}
pub mod prims {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    impl Solution {
        pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
            let mut n: usize = points.len();
            let mut adj: Vec<Vec<(i32, usize)>> = vec![vec![]; n];
            for i in 0..n {
                for j in i + 1..n {
                    let distance: i32 =
                        (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                    adj[i].push((distance, j));
                    adj[j].push((distance, i));
                }
            }

            let mut heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
            let mut visited: Vec<bool> = vec![false; n];
            let mut result: i32 = 0;
            heap.push((Reverse(0), 0));
            while let Some((Reverse(distance), index)) = heap.pop() {
                if visited[index] {
                    continue;
                }
                visited[index] = true;
                n -= 1;
                result += distance;
                if n == 0 {
                    return result;
                }
                for (dist, next) in adj[index].iter() {
                    if visited[*next] {
                        continue;
                    }
                    heap.push((Reverse(*dist), *next));
                }
            }

            -1i32
        }
    }
}
pub mod almost_prims {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashSet};
    impl Solution {
        pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
            let mut n: usize = points.len();

            let mut result: i32 = 0i32;
            let mut visited: HashSet<usize> = HashSet::new();
            let mut heap: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();

            heap.push((Reverse(0), 0));

            while let Some((Reverse(distance), index)) = heap.pop() {
                if !visited.insert(index) {
                    continue;
                }
                n -= 1;
                result += distance;
                if n == 0 {
                    return result;
                }

                for i in 0..points.len() {
                    if visited.contains(&i) {
                        continue;
                    }
                    heap.push((
                        Reverse(
                            points[index][0].abs_diff(points[i][0]) as i32
                                + points[index][1].abs_diff(points[i][1]) as i32,
                        ),
                        i,
                    ));
                }
            }

            result
        }
    }
}
