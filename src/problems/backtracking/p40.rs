pub struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(candidates.len() * target as usize);

        candidates.sort();

        let mut current_list: Vec<i32> = Vec::with_capacity(candidates.len());

        Self::dfs(0, &mut result, 0, &mut current_list, &candidates, target);

        result
    }

    pub fn dfs(
        index: usize,
        result: &mut Vec<Vec<i32>>,
        current_sum: i32,
        current_list: &mut Vec<i32>,
        candidates: &[i32],
        target: i32,
    ) {
        if current_sum == target {
            result.push(current_list.clone());
            return;
        }

        for i in index..candidates.len() {
            if i > index && candidates[i] == candidates[i - 1] {
                continue;
            }

            if current_sum.saturating_add(candidates[i]) > target {
                break;
            }

            current_list.push(candidates[i]);
            Self::dfs(
                i + 1,
                result,
                current_sum.saturating_add(candidates[i]),
                current_list,
                candidates,
                target,
            );
            current_list.pop();
        }
    }
}
