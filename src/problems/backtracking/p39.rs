pub mod backtracking {
    pub struct Solution;
    impl Solution {
        pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = Vec::with_capacity(150usize);

            Self::backtrack(&mut result, vec![], &candidates, target, 0usize);

            result
        }

        pub fn backtrack(
            result: &mut Vec<Vec<i32>>,
            mut current: Vec<i32>,
            nums: &Vec<i32>,
            target: i32,
            i: usize,
        ) {
            if target == 0 {
                result.push(current);
                return;
            }

            if i >= nums.len() || target < 0 {
                return;
            }

            current.push(nums[i]);
            Self::backtrack(result, current.clone(), nums, target - nums[i], i);
            current.pop();

            Self::backtrack(result, current.clone(), nums, target, i + 1);
        }
    }
}
