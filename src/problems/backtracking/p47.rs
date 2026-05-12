pub mod hashset {
    pub struct Solution;
    use std::collections::HashSet;
    impl Solution {
        pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
            let result: HashSet<Vec<i32>> = Self::backtrack(0usize, &nums).into_iter().collect();
            result.into_iter().collect()
        }

        pub fn backtrack(i: usize, nums: &Vec<i32>) -> Vec<Vec<i32>> {
            if i >= nums.len() {
                return vec![vec![]];
            }

            let mut result: Vec<Vec<i32>> = vec![];
            let perms: Vec<Vec<i32>> = Self::backtrack(i + 1, nums);

            for p in perms.into_iter() {
                for j in 0..p.len() + 1 {
                    let mut copy = p.clone();
                    copy.insert(j, nums[i]);
                    result.push(copy);
                }
            }

            result
        }
    }
}
