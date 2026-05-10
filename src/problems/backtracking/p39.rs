pub mod optimal_sort_prune {
    pub struct Solution;
    impl Solution {
        pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
            let mut result: Vec<Vec<i32>> = Vec::with_capacity(150usize);
            let mut nums = candidates;
            nums.sort();
            Self::dfs(&mut result, target, &mut vec![], 0i32, 0usize, &nums);

            result
        }

        pub fn dfs(
            result: &mut Vec<Vec<i32>>,
            target: i32,
            current: &mut Vec<i32>,
            sum: i32,
            i: usize,
            nums: &Vec<i32>,
        ) {
            if sum == target {
                result.push(current.clone());
                return;
            }

            if sum > target || i >= nums.len() {
                return;
            }

            for j in i..nums.len() {
                if sum + nums[j] > target {
                    return;
                }
                current.push(nums[j]);
                Self::dfs(result, target, current, sum + nums[j], j, nums);
                current.pop();
            }
        }
    }
}
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
