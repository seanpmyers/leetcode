pub mod dijkstra {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    impl Solution {
        pub fn find_critical_and_pseudo_critical_edges(
            n: i32,
            edges: Vec<Vec<i32>>,
        ) -> Vec<Vec<i32>> {
            let n = n as usize;
            let mut list: Vec<Vec<(usize, i32, usize)>> = vec![vec![]; n];

            for (i, edge) in edges.iter().enumerate() {
                let x: usize = edge[0] as usize;
                let y: usize = edge[1] as usize;
                let weight: i32 = edge[2];
                list[x].push((y, weight, i));
                list[y].push((x, weight, i));
            }

            let minimax = |source: usize, destination: usize, exclude: usize| -> i32 {
                let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
                let mut dist = vec![i32::MAX; n];

                dist[source] = 0i32;
                heap.push(Reverse((0i32, source)));

                while let Some(Reverse((max_weight, current))) = heap.pop() {
                    if current == destination {
                        return max_weight;
                    }
                    for &(y, weight, idx) in &list[current] {
                        if idx == exclude {
                            continue;
                        }
                        let weight = max_weight.max(weight);
                        if weight < dist[y] {
                            dist[y] = weight;
                            heap.push(Reverse((weight, y)));
                        }
                    }
                }

                i32::MAX
            };

            let mut critical: Vec<i32> = vec![];
            let mut pseudo: Vec<i32> = vec![];

            for (i, edge) in edges.into_iter().enumerate() {
                let x: usize = edge[0] as usize;
                let y: usize = edge[1] as usize;
                let weight: i32 = edge[2];
                let max = minimax(x, y, i);
                if weight < max {
                    critical.push(i as i32);
                    continue;
                }
                if weight <= max && max != i32::MAX {
                    pseudo.push(i as i32);
                }
            }

            vec![critical, pseudo]
        }
    }
}
