#[allow(unused_variables)]
pub mod dijkstra_hash_map {
    pub struct Solution;
    use std::collections::{BinaryHeap, HashMap};

    #[derive(PartialEq, PartialOrd)]
    struct State(f64, i32);

    impl Eq for State {}

    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
        }
    }

    impl Solution {
        pub fn max_probability(
            n: i32,
            edges: Vec<Vec<i32>>,
            succ_prob: Vec<f64>,
            start_node: i32,
            end_node: i32,
        ) -> f64 {
            let result: f64 = 0f64;
            if edges.is_empty() {
                return result;
            }
            let mut adj: HashMap<i32, Vec<(f64, i32)>> = HashMap::new();
            for (i, edge) in edges.into_iter().enumerate() {
                let source: i32 = edge[0];
                let destination: i32 = edge[1];
                let probability: f64 = succ_prob[i];
                adj.entry(source)
                    .and_modify(|x| x.push((probability, destination)))
                    .or_insert(vec![(probability, destination)]);
                adj.entry(destination)
                    .and_modify(|x| x.push((probability, source)))
                    .or_insert(vec![(probability, source)]);
            }

            let mut heap = BinaryHeap::new();
            let mut visited: HashMap<i32, f64> = HashMap::new();
            heap.push(State(1f64, start_node));

            while let Some(State(probability, source)) = heap.pop() {
                if visited.contains_key(&source) {
                    continue;
                }

                if source == end_node {
                    return result.max(probability);
                }

                visited
                    .entry(source)
                    .and_modify(|x| *x = x.max(probability))
                    .or_insert(probability);

                if let Some(pairs) = adj.get(&source) {
                    for p in pairs.iter() {
                        heap.push(State(probability * p.0, p.1));
                    }
                }
            }

            result
        }
    }
}
