pub mod array_dijkstra {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    impl Solution {
        pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
            let mut n: usize = n as usize;
            let mut adj: Vec<Vec<(i32, i32)>> = vec![vec![]; n + 1];

            for edge in times.into_iter() {
                let source: i32 = edge[0];
                let target: i32 = edge[1];
                let time: i32 = edge[2];
                adj[source as usize].push((target, time));
            }

            let mut visited: Vec<bool> = vec![false; n + 1];
            let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
            heap.push(Reverse((0, k)));

            while let Some(Reverse((time, node))) = heap.pop() {
                if visited[node as usize] {
                    continue;
                }
                visited[node as usize] = true;

                n -= 1;
                if n == 0 {
                    return time;
                }

                let list = &adj[node as usize];
                for (target, t) in list.iter() {
                    if visited[*target as usize] {
                        continue;
                    }
                    heap.push(Reverse((time + *t, *target)));
                }
            }

            -1i32
        }
    }
}
pub mod hashmap_dijkstra {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap, HashSet};
    impl Solution {
        pub fn network_delay_time(times: Vec<Vec<i32>>, mut n: i32, k: i32) -> i32 {
            let mut adj: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

            for edge in times.into_iter() {
                let source: i32 = edge[0];
                let target: i32 = edge[1];
                let time: i32 = edge[2];
                adj.entry(source).or_default().push((target, time));
            }

            let mut visited: HashSet<i32> = HashSet::new();
            let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
            heap.push(Reverse((0, k)));
            let mut result: i32 = 0;

            while let Some(Reverse((time, node))) = heap.pop() {
                if !visited.insert(node) {
                    continue;
                }

                n -= 1;
                if n == 0 {
                    return time;
                }

                let Some(list) = adj.get(&node) else {
                    result = result.max(time);
                    continue;
                };
                for (target, t) in list.iter() {
                    if visited.contains(target) {
                        continue;
                    }
                    heap.push(Reverse((time + *t, *target)));
                }
            }

            -1i32
        }
    }
}
