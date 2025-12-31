use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(dead_code)]
struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

#[allow(dead_code)]
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k: usize = k as usize;
        let mut heap: BinaryHeap<Reverse<i32>> = nums.iter().map(|x| Reverse(*x)).collect();

        for _ in k..heap.len() {
            heap.pop();
        }

        Self { heap: heap, k: k }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}
