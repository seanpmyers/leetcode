pub struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        if stones.len() == 1 {
            return stones[0];
        }
        let mut heap: BinaryHeap<i32> = BinaryHeap::from(stones);

        while heap.len() > 1 {
            let y: i32 = heap.pop().unwrap();
            let x: i32 = heap.pop().unwrap();
            let result: i32 = y.saturating_sub(x);
            if result > 0 {
                heap.push(result);
            }
        }

        heap.pop().unwrap_or(0)
    }
}
