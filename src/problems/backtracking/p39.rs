pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        pub const MAX_NUMBER_OF_UNIQUE_COMBINATIONS: usize = 150usize;
        let mut result: HashSet<Vec<i32>> =
            HashSet::with_capacity(MAX_NUMBER_OF_UNIQUE_COMBINATIONS);

        pub fn backtrack(
            mut current_sum: i32,
            current_list: &mut Vec<i32>,
            result: &mut HashSet<Vec<i32>>,
            candidates: &[i32],
            target: i32,
        ) {
            if result.len() == MAX_NUMBER_OF_UNIQUE_COMBINATIONS {
                return;
            }

            if current_sum >= target {
                if current_sum == target {
                    result.insert(current_list.to_vec());
                }
                return;
            }

            let Some(first) = candidates.first() else {
                return;
            };

            current_sum = current_sum.saturating_add(*first);
            current_list.push(*first);

            backtrack(current_sum, current_list, result, candidates, target);

            if let Some(previous) = current_list.pop() {
                current_sum = current_sum.saturating_sub(previous);
            };

            backtrack(current_sum, current_list, result, &candidates[1..], target);
        }

        let mut list = Vec::with_capacity(candidates.len());

        backtrack(0, &mut list, &mut result, &candidates, target);

        result.into_iter().collect()
    }
}
