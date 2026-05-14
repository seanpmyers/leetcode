use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
pub fn shortest_path(edges: Vec<Vec<i32>>, _n: i32, src: i32) -> HashMap<i32, i32> {
    let mut adj: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    for edge in edges.into_iter() {
        let source = edge[0];
        let destination = edge[1];
        let weight = edge[2];
        adj.entry(source)
            .and_modify(|x| x.push((weight, destination)))
            .or_insert(vec![(weight, destination)]);
    }

    let mut result: HashMap<i32, i32> = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(Reverse((0, src)));

    while let Some(Reverse((weight, source))) = heap.pop() {
        if result.contains_key(&source) {
            continue;
        }

        result
            .entry(source)
            .and_modify(|x| *x = weight)
            .or_insert(weight);

        if let Some(pairs) = adj.get(&source) {
            for (w, s) in pairs.iter() {
                heap.push(Reverse((*w + weight, *s)));
            }
        }
    }

    result
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn basic_test() {
        let edges = vec![vec![1, 2, 2], vec![1, 3, 4], vec![2, 3, 1]];
        let n = 3;
        let src = 1;

        let output = shortest_path(edges, n, src);

        assert_eq!(output.get(&1), Some(&0));
        assert_eq!(output.get(&2), Some(&2));
        assert_eq!(output.get(&3), Some(&3));
    }
}
