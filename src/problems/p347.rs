// #arrays #hashing
pub struct Solution {}
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 1 {
            return vec![nums[0]];
        }
        let mut count: HashMap<i32, i32> = HashMap::new();
        let mut heap: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
        for n in nums.iter() {
            count.entry(*n).and_modify(|c| *c += 1).or_insert(1);
        }
        let k: usize = k as usize;
        for (n, c) in count {
            heap.push((Reverse(c), n));
            if heap.len() > k {
                heap.pop();
            }
        }
        heap.into_iter().map(|x| x.1).collect::<Vec<i32>>()
    }
}
