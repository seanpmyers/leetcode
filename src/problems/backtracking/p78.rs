pub mod backtracking {
    pub struct Solution;
    impl Solution {
        pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
            use std::collections::HashSet;
            let mut result: HashSet<Vec<i32>> = HashSet::new();

            let number: usize = 0;

            pub fn backtrack(
                result: &mut HashSet<Vec<i32>>,
                current: &mut Vec<i32>,
                nums: &Vec<i32>,
                mut number: usize,
            ) {
                if !result.contains(current) && current.len() <= nums.len() {
                    result.insert(current.clone());
                }

                if number >= nums.len() || current.len() >= nums.len() {
                    return;
                }

                current.push(nums[number]);
                number = number.saturating_add(1);
                backtrack(result, current, nums, number);
                current.pop();
                backtrack(result, current, nums, number);
            }

            backtrack(&mut result, &mut Vec::new(), &nums, number);

            result.into_iter().collect()
        }
    }
}
