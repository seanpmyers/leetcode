// #arrays #hashing
pub struct Solution {}
use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        nums.iter().any(|value| !set.insert(*value))
    }
}
