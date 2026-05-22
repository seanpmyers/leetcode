pub mod dijkstra {
    pub struct Solution;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    impl Solution {
        pub fn find_cheapest_price(
            n: i32,
            flights: Vec<Vec<i32>>,
            src: i32,
            dst: i32,
            mut k: i32,
        ) -> i32 {
            let n: usize = n as usize;
            let dst: usize = dst as usize;
            let mut adj: Vec<Vec<(i32, usize)>> = vec![vec![]; n];
            for i in 0..flights.len() {
                let from: usize = flights[i][0] as usize;
                let to: usize = flights[i][1] as usize;
                let price: i32 = flights[i][2];
                adj[from].push((price, to));
            }
            let mut heap: BinaryHeap<(Reverse<i32>, usize, i32)> = BinaryHeap::new();
            let mut cost_to_node: Vec<i32> = vec![i32::MAX; n];
            heap.push((Reverse(0), src as usize, 0));
            k += 1;
            while let Some((Reverse(price), index, stops)) = heap.pop() {
                if index == dst && stops <= k {
                    return price;
                }

                if stops > cost_to_node[index] {
                    continue;
                }

                cost_to_node[index] = stops;

                if stops + 1 > k {
                    continue;
                }
                for (plus, next) in adj[index].iter() {
                    heap.push((Reverse(price + *plus), *next, stops + 1));
                }
            }

            -1i32
        }
    }
}
