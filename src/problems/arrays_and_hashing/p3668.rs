pub struct Solution;
impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let set: HashSet<i32> = HashSet::from_iter(friends.into_iter());
        order.into_iter().filter(|x| set.contains(&x)).collect()
    }
}
